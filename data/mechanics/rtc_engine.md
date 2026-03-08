# Pokemon Crystal -- RTC (Real Time Clock) Engine Reference

Source: pokecrystal engine/rtc/ (5 files, ~725 lines)

---

## RTC Hardware (rtc.asm)

### Hardware Interface
The Game Boy MBC3 cartridge has a built-in RTC with registers:
- RAMB_RTC_S: Seconds (0-59)
- RAMB_RTC_M: Minutes (0-59)
- RAMB_RTC_H: Hours (0-23)
- RAMB_RTC_DL: Day counter low byte
- RAMB_RTC_DH: Day counter high byte + control flags

### RTC DH Flags
- Bit 0 (B_RAMB_RTC_DH_DAY): Day counter bit 8
- Bit 6 (B_RAMB_RTC_DH_HALT): Halt RTC (stop counting)
- Bit 7 (B_RAMB_RTC_DH_CARRY): Day counter overflow (past 511 days)

### Core Operations

**StartRTC**: Latch clock, clear halt bit in DH register, close SRAM.

**StopRTC** (unreferenced): Set halt bit in DH register.

**GetClock**: Read all RTC registers into hRTCSeconds/Minutes/Hours/DayLo/DayHi.

**LatchClock**: Writing $01 then $00 to RTC latch register captures current time.

### Time of Day System (GetTimeOfDay)
Maps current hour to time of day:
```
Hours 4-9:   MORN (morning)
Hours 10-17: DAY
Hours 18-3:  NITE (night)
```

Constants: MORN_HOUR=4, DAY_HOUR=10, NITE_HOUR=18, MAX_HOUR=24

A beta time-of-day table exists (unreferenced) using different thresholds.

### Save/Load
**StageRTCTimeForSave**: Copy wCurDay, hHours, hMinutes, hSeconds to wRTC buffer.

**SaveRTC**: Clear carry flag in DH register, clear sRTCStatusFlags.

**StartClock**: GetClock, fix days, check for overflow, StartRTC.

---

## Clock Initialization (timeset.asm)

### InitClock
Prof. Oak's time-setting dialog at game start:
1. Ask for hour (0-23, default 10 AM)
   - D-pad up/down cycles through hours with wrapping
   - Displays time period (MORN/DAY/NITE) with adjusted 12-hour format
2. Confirm hour with Yes/No
3. Ask for minutes (0-59)
   - D-pad up/down cycles through minutes
4. Confirm minutes with Yes/No
5. Oak reacts based on time: morning="overslept", day="yikes", night="so dark"

### SetDayOfWeek
Day-of-week selection:
- 7 days (SUNDAY through SATURDAY)
- D-pad up/down cycles through days
- Confirm with Yes/No
- Weekday strings stored inline ("SUNDAY", "MONDAY", etc.)

### DST (Daylight Saving Time)
- InitialSetDSTFlag: Set DST bit in wDST, display adjusted time
- InitialClearDSTFlag: Clear DST bit, display adjusted time
- DST flag affects time calculation but is player-controlled

### Hour Display
AdjustHourForAMorPM: Converts 24-hour to 12-hour format.
- 0 (midnight) -> 12
- 1-12 -> as-is
- 13-23 -> subtract 12

---

## Clock Correction (restart_clock.asm)

### RestartClock
Triggered when RTC has overflowed or is invalid:
1. Display "Clock time may be wrong" message
2. Show day/hour/minute adjustment UI
3. D-pad up/down: Change selected value (wrapping)
4. D-pad left/right: Switch between day/hour/minute
5. Up/down arrows displayed at cursor position
6. Confirm with A, cancel with B

### Wraparound Values
| Field | Maximum | Arrow X Position |
|-------|---------|-----------------|
| Day | 7 (wraps to 0) | 4 |
| Hour | 24 (wraps to 0) | 12 |
| Minute | 60 (wraps to 0) | 15 |

---

## Day Fix System (rtc.asm)

### _FixDays
Checks RTC DH register for:
- Carry flag (day counter overflow past 511): Reset RTC
- Halt flag (RTC was stopped): Reset RTC

### FixDays
Normalizes the RTC day counter (which can go up to 511) to game day format:
- Days are wrapped modulo 140 (20 weeks)
- Game tracks wCurDay modulo 7 for day-of-week

### ClockContinue
Called when loading a save:
1. Check RTC status flags
2. If RTC_RESET or RTC_DAYS_EXCEED_255: Clear daily timers, increment mobile adapter counters
3. If RTC_DAYS_EXCEED_139: Compare saved day with current, update if needed
4. Otherwise: No action needed

### _InitTime
Calculates time offsets between RTC and player-set time:
- Subtracts RTC values from wStringBuffer2 (player-set time)
- Stores differences in wStartDay/Hour/Minute/Second
- These offsets are applied during UpdateTime to get game time

---

## Time-Dependent Systems (time.asm)

### Daily Reset Timer
CheckDailyResetTimer fires every 24 hours:
- Clears wDailyFlags1, wDailyFlags2, wSwarmFlags
- Clears wDailyRematchFlags (4 bytes)
- Clears wDailyPhoneItemFlags (4 bytes)
- Clears wDailyPhoneTimeOfDayFlags (4 bytes)
- Decrements wKenjiBreakTimer; if expired, resamples (random 3-6 days)
- Restarts 1-day countdown

### Phone Call Timer
Call delay system (see phone_engine.md):
- ReceiveCallDelay: Minutes-based countdown
- Decreasing delays: 20 -> 10 -> 5 -> 3 minutes

### Bug Catching Contest Timer
- BUG_CONTEST_MINUTES minutes, BUG_CONTEST_SECONDS seconds
- Counts down in real time using CalcSecsMinsHoursDaysSince
- Contest ends when timer reaches 0

### Lucky Number Show
Weekly event (every Friday):
- RestartLuckyNumberCountdown: Calculates days until next Friday
- _CheckLuckyNumberShowFlag: Returns carry when countdown expires

### Pokerus Tick
CheckPokerusTick: Each real day, applies Pokerus duration decrement to infected party Pokemon.

### Egg Steps
Every 256 steps (at step 128 specifically): DoEggStep decrements egg hatch counters.

### Mystery Gift Timer
Daily Mystery Gift limit reset tracked with sMysteryGiftTimer in SRAM.

### Time Calculation Helpers
CalcSecsMinsHoursDaysSince: Chain subtraction with borrow:
```
seconds_since = current_seconds - stored_seconds (+60 if borrow)
minutes_since = current_minutes - stored_minutes (+60 if borrow)
hours_since   = current_hours - stored_hours (+24 if borrow)
days_since    = current_day - stored_day (+140 if borrow)
```

---

## Key RAM Variables

| Variable | Description |
|----------|-------------|
| wCurDay | Current day (0-6, Sunday-Saturday) |
| hHours | Current hour (0-23) in HRAM |
| hMinutes | Current minute (0-59) in HRAM |
| hSeconds | Current second (0-59) in HRAM |
| wTimeOfDay | 0=morning, 1=day, 2=night |
| wStartDay/Hour/Minute/Second | Offset from RTC to game time |
| wDST | DST flag (bit 7) |
| wRTC | 4-byte saved RTC state (day, hour, min, sec) |
| hRTCSeconds through hRTCDayHi | Raw RTC register values in HRAM |
| wDailyResetTimer | Countdown for daily event reset |
| wLuckyNumberDayTimer | Countdown for Lucky Number Show |
| wBugContestMinsRemaining | Bug Contest minutes left |
| wBugContestSecsRemaining | Bug Contest seconds left |
| wKenjiBreakTimer | Kenji's break between calls (3-6 days) |

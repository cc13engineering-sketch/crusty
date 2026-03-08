# Pokemon Crystal — Time System

Source: pokecrystal disassembly (engine/rtc/rtc.asm, engine/overworld/time.asm, engine/events/, constants/misc_constants.asm)

---

## Time-of-Day Periods

| Period | Hours | Constant | Flag |
|--------|-------|----------|------|
| Morning | 04:00 - 09:59 | MORN_HOUR = 4 | MORN_F |
| Day | 10:00 - 17:59 | DAY_HOUR = 10 | DAY_F |
| Night | 18:00 - 03:59 | NITE_HOUR = 18 | NITE_F |

Source: `engine/rtc/rtc.asm` — GetTimeOfDay, TimesOfDay table

The TimesOfDay table is checked sequentially:
1. If hour < 4 (MORN_HOUR): Night
2. If hour < 10 (DAY_HOUR): Morning
3. If hour < 18 (NITE_HOUR): Day
4. If hour < 24 (MAX_HOUR): Night
5. Fallback: Morning

---

## RTC (Real Time Clock) Hardware

Pokemon Crystal uses the Game Boy cartridge's MBC3 Real Time Clock. The RTC provides:
- Seconds (0-59)
- Minutes (0-59)
- Hours (0-23)
- Days (0-511, 9-bit counter with overflow)

### RTC Registers
- RAMB_RTC_S: Seconds
- RAMB_RTC_M: Minutes
- RAMB_RTC_H: Hours
- RAMB_RTC_DL: Day counter (low 8 bits)
- RAMB_RTC_DH: Day counter (high bit) + halt flag + carry flag

### Clock Management
- LatchClock: Captures current RTC values for reading
- StartRTC: Clears the halt bit to resume clock
- StopRTC: Sets the halt bit to pause clock (unreferenced in game)
- SaveRTC: Stores current time state for save data
- StartClock: Called on game start — reads clock, fixes day overflow, starts RTC

### Day Overflow Handling
- The RTC day counter is 9 bits (0-511)
- If the carry bit (B_RAMB_RTC_DH_CARRY) is set, or if the halt bit is set, the RTC status is flagged as RTC_RESET
- FixDays: Handles when day count exceeds 140 (20 weeks)
- Game uses modular day tracking: wCurDay wraps via GetWeekday (day % 7)

Source: `engine/rtc/rtc.asm`, `home/time.asm`

---

## Daylight Saving Time (DST)

Mom can toggle DST via phone call or in-person visit.

**Setting DST:** Adds 1 hour to wStartHour. If this causes a day rollover (hour goes past 23), wStartDay increments.

**Unsetting DST:** Subtracts 1 hour from wStartHour. If this underflows (below 0), adds 24 and decrements wStartDay.

**Protection:** Cannot toggle DST if within 1 hour of midnight (would change the current day unpredictably).

Source: `engine/events/mom.asm` — DSTChecks, .SetClockForward, .SetClockBack

---

## Day-of-Week System

Days are determined by: `GetWeekday: wCurDay % 7`

| Value | Day |
|-------|-----|
| 0 | Sunday |
| 1 | Monday |
| 2 | Tuesday |
| 3 | Wednesday |
| 4 | Thursday |
| 5 | Friday |
| 6 | Saturday |

Source: `home/pokedex_flags.asm` — GetWeekday

---

## Day-of-Week Events

### Tuesday, Thursday, Saturday: Bug-Catching Contest
- **Location:** National Park
- **Time:** 9:00 AM - 6:00 PM (Morn/Day only)
- **Rules:** Given 20 Park Balls, keep one Pokemon from party, 20-minute time limit
- **Timer:** BUG_CONTEST_MINUTES = 20, BUG_CONTEST_SECONDS = 0
- **Scoring:** Based on rarity, level, and HP of caught Pokemon
- **Prizes:** 1st: Sun Stone, 2nd: Everstone, 3rd: Gold Berry

Source: `engine/events/bug_contest/contest.asm`, `engine/overworld/time.asm`

### Friday: Lapras in Union Cave
- **Location:** Union Cave B2F (requires Surf)
- **Level:** 20
- **Respawns:** Every Friday if previously defeated/caught

### Friday: Lucky Number Show
- **Location:** Goldenrod Radio Tower
- **Prizes:** Based on matching digits of OT IDs across all owned Pokemon
- **Reset:** Timer counts down days until next Friday using RestartLuckyNumberCountdown

Source: `engine/overworld/time.asm` — RestartLuckyNumberCountdown, .GetDaysUntilNextFriday

### Monday/Wednesday/Friday: Haircut Brothers
- **Location:** Goldenrod City Underground
- **Older Brother:** Available Monday, lower happiness gains but cheaper
- **Younger Brother:** Available certain days, higher max happiness gain but pricier

### Saturday: Goldenrod Dept Store Rooftop Sale
- **Location:** Goldenrod Department Store 6F
- **Special items available at discount**

### Sunday: Trainer Rematches
- Many phone-registered trainers are available for rematches on Sundays
- Phone contact rematch availability varies by trainer

---

## Time-Dependent Encounter Changes

### Wild Pokemon Availability
Different Pokemon appear at different times of day in the same route:
- **Morning-only:** Ledyba (route-specific), Pidgey (some routes)
- **Night-only:** Hoothoot, Spinarak, Gastly, Murkrow
- **Day-only:** Natu, Sunkern
- Morning/Day encounters use one table, Night encounters use a different table
- Some Pokemon have higher/lower encounter rates depending on time

### Headbutt Trees
- Different Pokemon available from headbutt trees at different times
- Heracross and Pineco availability varies by time and tree group

### Fishing
- Fishing encounter rates and species don't change by time of day

---

## Time-Dependent Evolution

| Pokemon | Evolves Into | Condition |
|---------|-------------|-----------|
| Eevee | Espeon | Happiness >= 220 + Level up during Morning/Day |
| Eevee | Umbreon | Happiness >= 220 + Level up during Night |

The evolution check reads wTimeOfDay and compares against MORN/DAY for Espeon and NITE for Umbreon.

Source: `engine/pokemon/evolve.asm` — EVOLVE_HAPPINESS method

---

## Daily Reset Timer

The game maintains a daily reset timer (wDailyResetTimer) that clears daily flags when a new day starts.

### Flags Reset Daily
- wDailyFlags1: Fruit tree availability, Shuckle gift, fish swarm
- wDailyFlags2: Additional daily events
- wSwarmFlags: Swarm encounter flags
- wDailyRematchFlags: Trainer rematch availability (4 bytes)
- wDailyPhoneItemFlags: Phone item gift flags (4 bytes)
- wDailyPhoneTimeOfDayFlags: Phone time-of-day flags (4 bytes)

### Kenji Break Timer
- Kenji (Blackbelt, Route 45) has a special break timer
- Random 3-6 day cooldown between his availability
- Tracked separately in wKenjiBreakTimer

Source: `engine/overworld/time.asm` — CheckDailyResetTimer, SampleKenjiBreakCountdown

---

## Pokerus Tick

- Pokerus duration decrements by 1 for each real-world day that passes
- Checked via CheckPokerusTick: compares wTimerEventStartDay against current day
- If at least 1 day has passed, calls ApplyPokerusTick to process each party member
- Pokerus strain determines duration (1-4 days)

Source: `engine/overworld/time.asm` — CheckPokerusTick

---

## Phone Call Timing

### Incoming Calls
- Receive call delay system with escalating frequency
- Initial delay: 20 minutes
- After 1st call: 10 minutes
- After 2nd call: 5 minutes
- After 3rd+ call: 3 minutes (minimum)
- Timer tracked in wReceiveCallDelay_MinsRemaining and wReceiveCallDelay_StartTime

### Outgoing Calls
- Player can call registered trainers at any time (ANYTIME flag)
- Trainers may or may not be available for rematches depending on day/time

Source: `engine/overworld/time.asm` — InitCallReceiveDelay, CheckReceiveCallTimer, .ReceiveCallDelays

---

## Mystery Gift Daily Reset

Mystery Gift has its own daily timer (sMysteryGiftTimer):
- Max 5 Mystery Gifts per day (MAX_MYSTERY_GIFT_PARTNERS)
- Max 1 gift from the same partner per day
- Timer checked via DoMysteryGiftIfDayHasPassed
- ResetDailyMysteryGiftLimitIfUnlocked called when timer expires

Source: `engine/overworld/time.asm` — DoMysteryGiftIfDayHasPassed

---

## Time-Based Music Changes

- Overworld music can vary by time of day in some locations
- National Park has a distinct night theme
- Route music is the same regardless of time (most routes)
- Indoor locations are time-independent for music

---

## Bug Contest Timer Details

The Bug Contest runs on a real-time countdown:
- Start: BUG_CONTEST_MINUTES minutes, BUG_CONTEST_SECONDS seconds
- Tracked via wBugContestStartTime (full day/hour/min/sec snapshot)
- Each check computes elapsed time via CalcSecsMinsHoursDaysSince
- Timer respects seconds and minutes independently
- If days or hours have elapsed, immediately times out (shouldn't happen in practice as the contest is 20 minutes)

Source: `engine/overworld/time.asm` — StartBugContestTimer, CheckBugContestTimer

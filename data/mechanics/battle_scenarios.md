# Pokemon Crystal Battle Scenarios — Exact Outcomes

Source: pokecrystal disassembly (engine/battle/core.asm, engine/battle/effect_commands.asm, engine/battle/move_effects/)

## Scenario 1: Both Pokemon Faint to Perish Song

**Setup:** Both Pokemon have Perish Song counter at 1. It's the start of the turn.
**Outcome:** Both Perish Song counters decrement to 0 during HandleBetweenTurnEffects. The game checks the player's Pokemon first (CheckFaint_PlayerThenEnemy or EnemyThenPlayer depending on who moved last). Whoever is checked first and faints "loses." In practice, the player's faint is checked first in most cases, so the player loses even though both fainted simultaneously.
**ASM:** HandleBetweenTurnEffects calls HandlePerishSong which sets HP to 0 for both. The subsequent faint check order determines the loser.

## Scenario 2: Explosion/Self-Destruct KOs the Last Enemy

**Setup:** Player uses Explosion against the opponent's last Pokemon. Both faint.
**Outcome:** The user faints first (the move sets the user's HP to 0 via BattleCommand_Selfdestruct before dealing damage). If the target also faints, the faint check order determines the outcome. In trainer battles, if the player's Explosion KOs the last enemy Pokemon, the player wins because the enemy ran out of Pokemon. The self-destruct HP reduction happens before damage calculation; the user is considered fainted even if the move misses.
**ASM:** BattleCommand_Selfdestruct halves the target's Defense, then sets user HP to 0. Damage is calculated and applied. Faint is checked after the move resolves.

## Scenario 3: Self-Destruct/Explosion Against Ghost Type

**Setup:** Player uses Explosion against a Ghost-type Pokemon.
**Outcome:** The user's HP is set to 0, the target's Defense is halved for the calculation, but the move deals 0 damage because Normal-type moves have no effect on Ghost types. The user faints for nothing. The Defense halving still occurs even though the move will miss due to type immunity.
**ASM:** BattleCommand_Selfdestruct runs before BattleCommand_Stab (type effectiveness check). The user's HP is already 0 when the type immunity message appears.

## Scenario 4: Fly/Dig vs Earthquake/Thunder

**Setup:** Pokemon A uses Fly on turn 1. Pokemon B uses Earthquake on turn 2 while A is in the air.
**Outcome:** Earthquake misses against a Pokemon using Fly (semi-invulnerable). Thunder hits a Pokemon using Fly. Earthquake hits a Pokemon using Dig (double damage). Gust/Twister hit a Pokemon using Fly (double damage). Surf hits a Pokemon using Dig (double damage).
**ASM:** CheckHit in effect_commands.asm checks SUBSTATUS_FLYING/SUBSTATUS_UNDERGROUND and compares against the specific move effects that can hit through semi-invulnerability.

## Scenario 5: Baton Pass Chain with Stat Boosts

**Setup:** Pokemon A uses Swords Dance (+2 Atk), then Baton Pass to Pokemon B. Pokemon B uses Agility (+2 Speed), then Baton Pass to Pokemon C.
**Outcome:** Pokemon C receives +2 Attack (from A) and +2 Speed (from B). Stat stage changes, confusion, Focus Energy, Substitute, Perish Song, Curse damage flag, Leech Seed, and Mean Look are all passed. ResetBatonPassStatus clears: Nightmare, Disable, Attraction, Transform, Encore, Wrap counts, and last move used.
**ASM:** BatonPass in baton_pass.asm copies all stat stages. ResetBatonPassStatus selectively clears specific substatus bits.

## Scenario 6: Transform + Level Up Move Learning

**Setup:** A Transformed Pokemon gains enough experience to level up and would learn a new move.
**Outcome:** The Pokemon attempts to learn the moves from its ORIGINAL species' learnset, not the Transformed species. Transform only changes the in-battle moveset; the underlying species data for level-up moves comes from the party data.
**ASM:** Level-up move learning uses wPartyMon data (which stores the real species), not wBattleMon data.

## Scenario 7: Destiny Bond + Pursuit

**Setup:** Pokemon A uses Destiny Bond, then switches out next turn. Pokemon B uses Pursuit as A switches.
**Outcome:** Pursuit hits with doubled damage as A switches. If A faints from Pursuit, Destiny Bond does NOT activate because Destiny Bond's effect wears off at the start of the turn before the switch happens. Destiny Bond is cleared when the user takes any action (including switching).
**ASM:** Destiny Bond checks SUBSTATUS_DESTINY_BOND which is cleared at the start of the turn in CheckTurn.

## Scenario 8: Encore Forcing Struggle

**Setup:** Pokemon A is Encored into a move that has 0 PP remaining.
**Outcome:** The Pokemon is forced to use Struggle. Encore forces the selection of the locked move, but if that move has 0 PP, the game falls through to Struggle. Encore does NOT end early because of 0 PP — it continues counting down normally.
**ASM:** The move selection checks PP after Encore forces the move choice. If PP is 0, Struggle is used instead.

## Scenario 9: All Moves at 0 PP

**Setup:** A Pokemon has exhausted PP for all 4 moves during battle.
**Outcome:** The Pokemon is forced to use Struggle. Struggle has 10 base power, is typeless (Normal), and the user takes recoil equal to 1/4 of the damage dealt. There is no PP check for Struggle — it can be used indefinitely.
**ASM:** When all PP is 0, the move selection defaults to STRUGGLE. Struggle's effect includes recoil calculation.

## Scenario 10: Thick Club + Critical Hit Overflow

**Setup:** Marowak holds Thick Club, has high Attack, uses a high-power physical move, and gets a critical hit.
**Outcome:** The damage can overflow. Thick Club doubles Attack, which can cause the stat to exceed 1024. During damage calculation, if the intermediate value exceeds $FFFF (65535), it wraps around via TruncateHL_BC, potentially resulting in very low damage instead of very high damage.
**ASM:** ThickClubBoost in effect_commands.asm doubles wCurDamage via sla/rl. TruncateHL_BC then caps at $FFFF but the damage formula's intermediate product can overflow 16 bits.

## Scenario 11: Badge Boost Stacking Glitch

**Setup:** Player Pokemon has a type matching one of their badges. The Pokemon uses a physical move.
**Outcome:** DoBadgeTypeBoosts applies a 1.125x multiplier for each matching badge type. But due to the way the stats are modified in RAM (wBattleMonAttack etc.) and recalculated on certain events, the boost can be re-applied multiple times in a single battle without being reset, causing stats to grow uncontrollably.
**ASM:** DoBadgeTypeBoosts in misc.asm modifies wBattleMon stats directly. Stats are not reset to base between applications in certain edge cases.

## Scenario 12: Sleep Counter Reset on Switch

**Setup:** Pokemon A is put to sleep (sleep counter 1-7). Player switches to Pokemon B, then back to A.
**Outcome:** In Gen 2, when a sleeping Pokemon is switched out and back in, the sleep counter is preserved from the party data. However, there is a quirk: the sleep counter is written to party data when switching out, but the battle mon's counter may differ from the party mon's counter due to the way Sleep Talk and Snore interact with the counter.
**ASM:** Sleep counter is stored in the low 3 bits of MON_STATUS. Party/battle sync happens during switch operations.

## Scenario 13: Protect/Detect Success Rates

**Setup:** Pokemon uses Protect on consecutive turns.
**Outcome:** First use: always succeeds. Second consecutive use: 1/3 chance (255/3 = 85, random < 85). Third consecutive: 1/7 (255/7 = 36). Fourth: 1/15. Pattern: success threshold = 255 / (2^N - 1) where N = consecutive uses. If the move fails, the counter resets.
**ASM:** wProtectCount is checked in BattleCommand_Protect. Random value compared against (255 >> ProtectCount).

## Scenario 14: Focus Energy + Critical Hit Interaction

**Setup:** Pokemon uses Focus Energy.
**Outcome:** In Gen 2, Focus Energy correctly adds +1 to the critical hit stage (unlike Gen 1 where it bugged and divided the crit rate by 4). With Focus Energy and a high-crit move (like Slash), the Pokemon reaches stage 3 (1/3 crit rate). Maximum stage 4+ gives 1/2 crit rate.
**ASM:** BattleCommand_Critical checks SUBSTATUS_FOCUS_ENERGY and adds 1 to the crit stage if set.

## Scenario 15: Substitute Blocks Status but Not Stat Changes

**Setup:** Pokemon A has a Substitute up. Opponent uses Growl (stat-lowering move).
**Outcome:** Growl goes through the Substitute and lowers Attack. Most status moves (Thunder Wave, Toxic, etc.) are blocked by Substitute. However, stat-lowering sound-based moves and some specific effects can bypass it. The key rule: moves that deal damage are blocked if they wouldn't break the sub, and status moves are blocked — but the implementation is inconsistent with some moves getting through.
**ASM:** SubstituteCheck in effect_commands.asm is called for specific effects. Not all non-damaging moves check for Substitute.

## Scenario 16: Counter vs Mirror Coat — Wrong Type Check

**Setup:** Player's Pokemon uses Counter. Enemy used a special move last turn.
**Outcome:** Counter fails because it only works against physical moves (types < SPECIAL threshold). Mirror Coat only works against special moves (types >= SPECIAL). The type of the move determines physical/special, not the move's category tag. In Gen 2, all types are either physical or special: Normal, Fighting, Flying, Poison, Ground, Rock, Bug, Ghost, Steel are physical. Fire, Water, Grass, Electric, Psychic, Ice, Dragon, Dark are special.
**ASM:** Counter checks `cp SPECIAL` against the move's type. Mirror Coat checks if type >= SPECIAL.

## Scenario 17: Counter/Mirror Coat After Using an Item

**Setup:** Enemy uses a physical attack. Next turn, player uses a Potion. Enemy uses Counter.
**Outcome:** BUG: Counter/Mirror Coat still work even if the opponent used an item instead of a move. The game stores the last damage dealt and the move type, and these aren't cleared when an item is used. So Counter can reflect damage from a move used 2+ turns ago if an item was used in between.
**ASM:** wCurDamage and the move type byte are not reset when items are used in battle.

## Scenario 18: Perish Song + Spikes — 0 HP Ghost

**Setup:** Pokemon with Perish Song count at 1 faints. Opponent sends in a new Pokemon. Spikes are on the opponent's side.
**Outcome:** BUG: If a Pokemon faints to Perish Song and the replacement takes Spikes damage that would bring it to exactly 0 HP, the game continues with the Pokemon at 0 HP. The faint check after Spikes damage has a bug where it doesn't properly trigger in this specific sequence.
**ASM:** The faint handling after HandlePerishSong and the subsequent switch-in Spikes check have an ordering issue documented in bugs_and_glitches.md.

## Scenario 19: Lock-On + Fly/Dig

**Setup:** Pokemon A uses Lock-On. Next turn, Pokemon B uses Fly. Pokemon A attacks while B is in the air.
**Outcome:** Lock-On guarantees the next move hits, bypassing accuracy checks entirely — including the semi-invulnerability of Fly/Dig. Any move used by A will hit B regardless of being airborne/underground.
**ASM:** CheckHit checks SUBSTATUS_LOCK_ON before checking SUBSTATUS_FLYING/UNDERGROUND. Lock-On flag takes priority.

## Scenario 20: 1/256 Effect Miss Glitch

**Setup:** Pokemon uses Thunder Wave (100% accuracy, no secondary effect chance to worry about) against a target.
**Outcome:** Even with 100% accuracy, there's a 1/256 chance the move misses. The accuracy check generates a random byte 0-255 and compares it to the calculated accuracy value. If the random byte equals exactly 255 and accuracy is 255 (100%), the comparison uses `cp` which treats 255 as "not less than 255," causing a miss.
**ASM:** In BattleCommand_CheckHit, `call BattleRandom; cp b` where b=255 means random value 255 causes failure because `cp 255` clears carry (255 is not < 255).

## Scenario 21: Present Healing the Opponent

**Setup:** Pokemon uses Present.
**Outcome:** 40% chance of 40 power, 30% chance of 80 power, 10% chance of 120 power, 20% chance of healing the opponent by 1/4 of their max HP. The healing variant ignores type immunity — it still heals Ghost types even though Present is Normal-type. The damage variants work normally with type effectiveness.
**ASM:** PresentPower table in present_power.asm. The heal branch skips damage calculation entirely and directly adds HP.

## Scenario 22: Belly Drum at Exactly 50% HP

**Setup:** Pokemon has exactly 50% of its max HP remaining. It uses Belly Drum.
**Outcome:** Belly Drum succeeds. It costs exactly 50% of max HP and maximizes Attack stat stage to +6. If HP is less than 50% (even by 1), it fails.
**ASM:** BellyDrum checks if current HP >= half max HP. BUG: It calls AttackUp2 BEFORE the HP check, so +2 Attack can be applied even if the HP check subsequently fails.

## Scenario 23: Return at 0 Happiness / Frustration at 255 Happiness

**Setup:** Pokemon with 0 happiness uses Return. Pokemon with 255 happiness uses Frustration.
**Outcome:** BUG: Both calculate to 0 base power and deal 0 damage. Return power = happiness * 10 / 25. At happiness 0: 0 * 10 / 25 = 0. Frustration power = (255 - happiness) * 10 / 25. At happiness 255: 0 * 10 / 25 = 0. The formula rounds down to 0 in both edge cases, dealing no damage.
**ASM:** return.asm and frustration.asm multiply by 10 then divide by 25. Integer division of 0 is 0.

## Scenario 24: Beat Up Against Substitute

**Setup:** Pokemon uses Beat Up against a target with a Substitute.
**Outcome:** BUG: Each hit of Beat Up uses a different party member's Attack stat and species' base Attack. The Substitute check only happens once at the beginning. If the Sub breaks on an early hit, subsequent hits deal damage directly to the Pokemon behind the broken Sub.
**ASM:** Beat Up loops through party members. The Sub HP is decremented in the loop without re-checking if it's already broken.

## Scenario 25: Magnitude Underground (Dig)

**Setup:** Pokemon B is using Dig (underground). Pokemon A uses Magnitude.
**Outcome:** Magnitude hits Pokemon underground for double damage, same as Earthquake. The magnitude number is still randomly determined (4-10), and the doubled damage applies to whatever the rolled power was.
**ASM:** Magnitude has EFFECT_EARTHQUAKE which includes the double damage against underground targets.

## Scenario 26: Fury Cutter Maximum Damage

**Setup:** Pokemon uses Fury Cutter successfully 5 times in a row.
**Outcome:** Fury Cutter's power doubles each consecutive successful hit, capping at 5 doublings. Base power 10 -> 20 -> 40 -> 80 -> 160. The 6th hit and beyond stay at 160 (5 doublings). If any hit misses or a different move is used, the counter resets.
**ASM:** FuryCutter caps at 5 via `cp 6` check. Damage doubled via sla/rl with $FFFF overflow cap.

## Scenario 27: Mean Look + Baton Pass

**Setup:** Pokemon A uses Mean Look on opponent. Pokemon A then uses Baton Pass to Pokemon B.
**Outcome:** The Mean Look trapping effect IS passed via Baton Pass. Pokemon B now traps the same opponent. The opponent cannot switch. This is intentional behavior, not a bug. Other trapping conditions (Wrap, Fire Spin, etc.) are NOT passed because their wrap counts are cleared by ResetBatonPassStatus.
**ASM:** Mean Look sets SUBSTATUS_CANT_RUN which is a battle substatus that Baton Pass does NOT clear.

## Scenario 28: Rollout + Defense Curl Combo

**Setup:** Pokemon uses Defense Curl, then starts Rollout.
**Outcome:** Defense Curl sets SUBSTATUS_CURLED. When Rollout's damage calculation reads this flag, it effectively adds +1 to the doubling counter. So Rollout's progression becomes: 30 -> 60 -> 120 -> 240 -> 480 power equivalents instead of 30 -> 60 -> 120 -> 240 -> 480 (base power 30 with up to 16x multiplier instead of 8x). MAX_ROLLOUT_COUNT is 5 hits regardless.
**ASM:** rollout.asm checks SUBSTATUS_CURLED and increments the roll counter before the power doubling loop.

## Scenario 29: Future Sight Damage Timing

**Setup:** Turn 1: Pokemon A uses Future Sight. Turn 2: Normal move. Turn 3: Future Sight hits.
**Outcome:** Future Sight calculates damage at the time of USE (Turn 1), not at the time of HIT (Turn 3). The stored damage in wPlayerFutureSightDamage is locked in on Turn 1. If the user switches out, the attack still hits on Turn 3. Future Sight counts down from 4, triggers at 1. It cannot be stacked — using it again while one is pending fails.
**ASM:** future_sight.asm stores damage in wPlayerFutureSightDamage. HandleBetweenTurnEffects decrements wPlayerFutureSightCount and applies stored damage when it reaches 1.

## Scenario 30: Weather + SolarBeam Interaction

**Setup:** Sunny Day is active. Pokemon uses SolarBeam.
**Outcome:** SolarBeam charges instantly in Sun (skips the charge turn). In Rain, SolarBeam still requires the charge turn AND its power is halved to 60. In Sandstorm, SolarBeam functions normally (2-turn charge, full power).
**ASM:** The charge skip is in the SolarBeam effect handler. The power halving in Rain is in DoWeatherModifiers which applies WEATHER_RAIN + SOLARBEAM = 0.5x multiplier.

## Scenario 31: Wild Pokemon Using Teleport

**Setup:** Wild Abra uses Teleport.
**Outcome:** The wild Pokemon flees the battle. Teleport in wild battles functions as the wild Pokemon escaping. In trainer battles, Teleport by the enemy fails. The player using Teleport in a wild battle successfully flees.
**ASM:** Teleport's effect in wild battles triggers the flee mechanic, equivalent to the wild Pokemon running.

## Scenario 32: Catching a Transformed Pokemon

**Setup:** Wild Ditto uses Transform to become your Pikachu. You catch the Ditto.
**Outcome:** BUG: You catch a Ditto, not a Pikachu, but there's a subtle issue. The caught Pokemon's data correctly reverts to Ditto, but the Pokedex registration and display may show incorrect information in certain edge cases. The base species is always preserved for the catch.
**ASM:** bugs_and_glitches.md documents "Catching a Transformed Pokemon always catches a Ditto" as expected behavior with some quirks.

## Scenario 33: Whiteout Money Loss

**Setup:** Player has 50,000 money and all Pokemon faint.
**Outcome:** Player loses exactly half their money: 25,000. The money is halved via a right shift of the 3-byte money value. Player respawns at the last Pokemon Center (or home).
**ASM:** HalveMoney in whiteout.asm performs `srl [hli]; rra [hli]; rra [hl]` on the 3-byte wMoney.

## Scenario 34: Pay Day Money Collection

**Setup:** Pokemon with Amulet Coin uses Pay Day. Pokemon is level 50.
**Outcome:** Pay Day scatters coins equal to 2x the user's level per use (100 coins for level 50). At the end of battle, if Amulet Coin is held, the total Pay Day money is doubled again. So total = level * 2 * number_of_pay_day_uses * 2 (with Amulet Coin). Capped at 3-byte max ($FFFFFF = 16,777,215 but practically limited by 6-digit display).
**ASM:** BattleCommand_PayDay adds level*2 to wPayDayMoney. CheckPayDay at battle end doubles if wAmuletCoin is set.

## Scenario 35: Multi-Hit Move (Double Slap) Distribution

**Setup:** Pokemon uses DoubleSlap.
**Outcome:** Hit distribution: 2 hits = 37.5%, 3 hits = 37.5%, 4 hits = 12.5%, 5 hits = 12.5%. Each hit is independently checked for critical hits. The move shows total damage after all hits. If any hit breaks a Substitute, remaining hits stop.
**ASM:** The multi-hit count is determined by random byte: 0-95 = 2 hits, 96-191 = 3 hits, 192-223 = 4 hits, 224-255 = 5 hits.

## Scenario 36: Struggle Recoil

**Setup:** Pokemon is forced to use Struggle.
**Outcome:** Struggle is typeless (Normal type in Gen 2), 50 power, and inflicts 1/4 of the damage dealt as recoil to the user. Unlike Explosion, Struggle doesn't set user HP to 0 — it only causes the recoil damage. If Struggle KOs the opponent, the user still takes recoil. Struggle has no type effectiveness — it deals neutral damage to all types including Ghost.
**ASM:** Struggle uses EFFECT_RECOIL_HIT. Being typeless means it bypasses Ghost immunity since it's a special case.

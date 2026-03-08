# Pokemon Crystal -- Complete Bugs and Glitches Reference

Source: pokecrystal docs/bugs_and_glitches.md (official), cross-referenced with our existing glitches_quirks.md.

---

## Multi-Player Battle Engine Bugs

### 1. Perish Song + Spikes 0 HP Bug
When a Pokemon faints from Perish Song and replacement takes Spikes damage that would KO it, the replacement can be left at 0 HP without fainting ("zombie" Pokemon). The faint check only runs once per phase.

### 2. Thick Club / Light Ball Stat Overflow
When Cubone/Marowak holds Thick Club or Pikachu holds Light Ball, (Special) Attack is doubled via `sla l; rl h`. If stat exceeds 1024, it wraps around to a tiny value, dramatically REDUCING damage instead of boosting it. Triggered by Swords Dance + Thick Club at high levels.

### 3. Metal Powder Defense Overflow
Metal Powder (Ditto's item) adds 50% to (Special) Defense. If the boosted value exceeds 1024, the same wrap-around bug as Thick Club applies, potentially INCREASING damage taken.

### 4. Reflect / Light Screen Defense Overflow
Reflect/Light Screen double Defense/SpDef. In link battles, the overflow fix is disabled for Gold/Silver compatibility, so doubling above 1024 wraps around. Single-player has a partial fix.

### 5. 1/256 Secondary Effect Miss
Moves with 100% secondary effect chance generate random 0-255 and compare with `cp [hl]`. When random = 255, comparison fails. ~0.4% of "guaranteed" secondary effects don't trigger. Affects all moves with any secondary effect chance.

### 6. Belly Drum HP Glitch
Belly Drum calls AttackUp2 BEFORE checking if user has 50% HP. If Attack can be boosted but HP is below 50%, the +2 Attack boost happens free without HP cost, then "But it failed!" prints.

### 7. Berserk Gene Confusion Duration
Berserk Gene doesn't properly set confusion counter. Uses either 256 turns (if counter was 0) or the PREVIOUS Pokemon's confusion count instead of generating random 2-5 turns.

### 8. Confusion Damage Boosted by Items
Self-hit confusion damage passes through full BattleCommand_DamageCalc, including type-boosting item checks and Explosion defense-halving. Confused Pokemon holding type-boosting items deal more self-damage.

### 9. Defense-Lowering After Substitute Break
Acid, Iron Tail, and Rock Smash can lower Defense after breaking a Substitute. The secondary effect check runs even though the target's Substitute just broke. (effectchance is called twice in the move script.)

### 10. Counter/Mirror Coat Item Bug
Counter and Mirror Coat don't clear wLastCounterMove when opponent uses an item. If opponent attacks turn 1, then uses Potion turn 2, Counter on turn 2 still reflects turn-1 damage.

### 11. Disabled PP Up Move Doesn't Trigger Struggle
If a move is Disabled but has PP boosted by PP Up, the PP check uses `and a` instead of `and PP_MASK`, so the PP Up bits can make non-zero PP appear even when actual PP is 0, preventing Struggle.

### 12. Pursuit Faint Preserves Status
If a Pokemon faints from Pursuit while switching, it retains its status condition (burn, poison, etc.) in party data. When revived, it still has the status.

### 13. Lock-On/Mind Reader vs Fly/Dig
Lock-On/Mind Reader guarantee next move hits, but CheckHiddenOpponent doesn't check Lock-On substatus. Status moves (Attract, Curse, Foresight, Mean Look, Mimic, Nightmare, Spider Web, Transform, stat-lowering) still miss during Fly/Dig semi-invulnerable turns.

### 14. Beat Up Link Desync
Beat Up compares wCurBeatUpPartyMon with `[hl]` instead of `c`, causing potential desynchronization in link battles.

### 15. Beat Up Single Pokemon Bug
If the user has only one Pokemon in party, Beat Up prevents Substitute from being raised and King's Rock from working.

### 16. Beat Up King's Rock False Trigger
Beat Up may trigger King's Rock flinch even if all hits failed.

### 17. Present Link Battle Damage
Present damage calculation has different code paths for link vs single-player. The link version skips push/pop of bc/de registers, corrupting the damage value.

### 18. Return/Frustration Zero Damage
Return deals no damage when happiness is 0-2; Frustration deals no damage when happiness is 253-255. The formula `happiness * 10 / 25` rounds to 0 for these edge values.

### 19. Dragon Scale vs Dragon Fang
Dragon Scale (trade evolution item) boosts Dragon-type moves by ~10%. Dragon Fang (intended type booster) has no effect. Item attribute table has them swapped.

### 20. Switch Freeze with Max HP < 4
Switching calculates enemy HP percentage using `max_HP / 4` as denominator. If max HP < 4, this divides by zero, entering an infinite loop that freezes the game.

### 21. Stat-Boosting Damage Moves Skip Boost After KO
Moves that deal damage AND boost stats (AncientPower, Steel Wing) have the stat boost after `checkfaint`, so if the opponent faints, the stat boost is skipped.

---

## Single-Player Battle Engine Bugs

### 22. Transform + Sketch Exploit
A Transformed Pokemon can use Sketch because the code checks `SUBSTATUS_TRANSFORMED` on the OPPONENT instead of the USER. This allows learning otherwise unobtainable moves.

### 23. Catching Transformed Pokemon = Ditto
Catching a Transformed Pokemon always catches a Ditto regardless of original species.

### 24. Experience Underflow at Level 1
Medium-Slow growth formula produces negative experience at level 1: EXP(1) = -54, which wraps to 16,777,162 as unsigned. This is why eggs hatch at level 5.

### 25. Dude Tutorial Crash
The Dude's catching tutorial can crash if party and PC box are both full when it starts.

### 26. BRN/PSN/PAR Don't Affect Catch Rate
Catch rate status bonus: code checks for FRZ/SLP but then skips the BRN/PSN/PAR check due to `and a` checking wrong register. Only Sleep (+10) and Freeze (+10) work.

### 27. Moon Ball Bug
Moon Ball checks for BURN_HEAL ($3C) instead of MOON_STONE. Also reads wrong offset in evolution data. Result: 1x catch rate for everything.

### 28. Love Ball Gender Bug
Love Ball checks for same gender instead of opposite gender. Boosts catch rate when genders MATCH (wrong direction).

### 29. Fast Ball Bug
Fast Ball's flee check uses `jr nz, .next` instead of `jr nz, .loop`, only matching Magnemite, Grimer, and Tangela (which don't flee). Actual fleeing Pokemon get no bonus.

### 30. Heavy Ball Weight Bug
Three Pokemon (Kadabra #64, Tauros #128, Sunflora #192) have wrong Pokedex entry bank because species ID isn't decremented before the bank calculation.

### 31. Catch Rate Overflow for HP > 341
`3 * HPmax` overflows 16 bits when max HP > 341. High-level Pokemon can have incorrect catch rates.

### 32. PRZ/BRN Stat Reductions Don't Apply on Switch
When an enemy Pokemon switches in with burn or paralysis, the stat reductions aren't applied because `ApplyStatusEffectOnEnemyStats` isn't called.

### 33. Glacier Badge SpDef Boost Bug
Glacier Badge should boost both Special Attack and Special Defense. Due to register overwrite in CheckBadge, SpDef boost only applies if unboosted SpAtk is 206-432 or 661+.

### 34-39. AI Bugs
- **Mean Look**: Smart AI checks own Pokemon's toxic status instead of opponent's
- **Conversion2**: Discouraged after first turn (logic inverted)
- **Sunny Day**: Doesn't recognize SolarBeam, Flame Wheel, Moonlight as sun-benefiting moves
- **Cautious AI residual**: 90% discourage fails to loop correctly, may stop early
- **Future Sight**: AI doesn't check if Future Sight is already pending
- **CheckTypeMatchup**: AI passes registers incorrectly to type checking function

### 40-43. AI Item Bugs
- Full Heal/Full Restore don't cure Nightmare status
- Full Heal doesn't cure confusion
- Full Heal/Full Restore don't restore Attack/Speed drops from BRN/PAR
- AI base reward value can be mistakenly used as an item index

### 44. Wild Pokemon Teleport
Wild Pokemon always succeed at Teleport regardless of level difference. The comparison is inverted.

### 45. RIVAL2 Lower DVs
RIVAL2 (later encounters) has DVs of 9/8/8/8 vs RIVAL1's 13/13/13/13. Rival gets weaker as game progresses.

### 46. HELD_CATCH_CHANCE No Effect
The catch rate held item effect doesn't work because `ld b, a` is missing before calling GetItemHeldEffect.

### 47. Credits Move Menu Bug
After credits sequence, hInMenu is left at TRUE, changing battle move selection from press-release to continuous hold scrolling.

---

## Overworld Engine Bugs

### 48. LoadMetatiles 128 Block Limit
`add a` before shifting causes wrap-around past 128 blocks, preventing use of blocksets with more than 128 metatiles.

### 49. Surfing Map Connection Bug
Surfing directly across a map connection doesn't load the new map because the movement script uses `applymovement` which doesn't trigger connection checks.

### 50. Swimming NPC Movement Radius
Swimming NPCs (SPRITEMOVEDATA_SWIM_WANDER) ignore their movement radius due to the NOCLIP_TILES flag check.

### 51. Fishing on NPCs
You can fish on top of NPCs because the code doesn't check for a facing object after verifying the tile is water.

### 52. Day-Care Experience Loss
Withdrawing from Day-Care resets experience to minimum for current level. If the Pokemon gained no levels, it may lose experience.

---

## Graphics Bugs

### 53. Ellipsis Too High
The "..." ellipsis tile in battle HP bar is drawn 2 pixels too high.

### 54. Port Tileset Corner Tiles
Left-hand warp carpet corner tiles in port tileset are drawn incorrectly.

### 55. Ruins of Alph Roof Color
Ruins of Alph Outside has poorly-chosen roof colors (default gray + Cinnabar night red mix).

### 56. Slowpoke Well Corner Tile
Block $5B uses tile $4B instead of $47 for bottom-left stone corners.

### 57. Unown Egg Letter Bug
Hatching Unown egg shows wrong letter because it reads DVs from wBattleMonDVs instead of the party mon's DVs.

### 58. Beat Up Substitute After Protect
Beat Up fails to raise Substitute if blocked by Protect/Detect.

### 59. HP Bar Slow Animation
HP bar animation speed doesn't account for high HP values, making it extremely slow for high-HP Pokemon.

### 60. HP Bar Off-by-One
HP bar animation has off-by-one error at low HP values.

### 61. Park Ball Corrupt Animation
Using a Park Ball outside Bug Catching Contest has a corrupt animation.

### 62. Battle Transition Level Bug
Battle transitions don't account for enemy level. wEnemyMonLevel isn't initialized yet, and wBattleMonLevel gets overwritten.

### 63. Inconsistent Trainer Sprites
Several trainer NPCs have incorrect overworld sprites or palettes (PsychicRodney, FisherAndre/Raymond, HikerKenny, etc.)

### 64. Tackle Animation Missing Part
Tackle animation copies 2 rows instead of 1, hitting the horizontal sprite limit.

---

## Audio Bugs

### 65. Slot Machine Payout SFX
Payout sound effects cut each other off due to inverted check (`ret z` should be `ret nz`).

### 66. Team Rocket Music Missing
Rocket battle music doesn't play for Executives or Scientists, only Grunts.

### 67. No Bump Noise on Tile $3E
Standing direction ($FF) indexes into edge warp table, hitting $3E, suppressing bump sound on that tile.

### 68. Entei Cry Distortion
Playing Entei's Pokedex cry can distort Raikou's and Suicune's cries.

### 69. SFX_RUN Incorrect Playback
Wild Pokemon flee SFX uses PlaySFX instead of WaitPlaySFX, cutting off the sound.

---

## Text Bugs

### 70. Five-Digit EXP Display
Experience gain text uses 4-digit format instead of 5-digit, truncating large gains.

### 71. Stone Evolution Compatibility
Only the first three evolution entries can have Stone compatibility reported correctly due to buffer size limit.

### 72. EVOLVE_STAT Stone Reporting
EVOLVE_STAT entries use 4 bytes instead of 3, breaking Stone compatibility loop.

### 73. HOF Master Title Inaccessible
The "HOF Master!" title for 200-time Hall of Famers uses `cp HOF_MASTER_COUNT + 1` instead of `cp HOF_MASTER_COUNT`, making it unreachable.

---

## Scripted Event Bugs

### 74. Clair Double TM24
Clair can give TM24 Dragonbreath twice due to missing scene flag reset.

### 75. Daisy Grooming 0.4% Fail
Daisy's grooming has 0.4% chance of not increasing happiness due to $FF - $FF failing to set carry.

### 76. Lake of Rage Magikarp Shorter
Magikarp in Lake of Rage are SHORTER, not longer. The map group comparison is inverted.

### 77. Magikarp Length Unit Errors
Length comparisons use wrong values: `HIGH(1536)` = 6 should be 5 feet, `LOW(1616)` = 80 should be 4 inches, etc.

### 78. Magikarp Length Miscalculation
BCLessThanDE has `ret nc` after `cp d` that should not be there, skipping the low byte comparison.

### 79. CheckOwnMon 5-Letter OT
Only checks first 5 letters of OT names (Japanese name length), not full player name length.

### 80. CheckOwnMonAnywhere Skips Day-Care
Doesn't check Day-Care when looking for owned Pokemon, preventing Ho-Oh encounter if legendary beast is deposited.

### 81. Unused phonecall Crash
phonecall script command calls BrokenPlaceFarString which switches banks unsafely.

### 82. Mania Shuckie Dialogue
Wrong dialogue triggers when trying to return Shuckie with no other Pokemon.

---

## Internal Engine Bugs

### 83. Mid-Save Corruption
Saves corrupted by mid-save shutoff are not detected or handled. Allows Pokemon duplication.

### 84. ScriptCall Stack Overflow
ScriptCall doesn't check wScriptStackSize before pushing, can overflow wScriptStack and crash.

### 85. LoadSpriteGFX Capacity Overflow
LoadSpriteGFX doesn't limit UsedSprites capacity, and GetSprite clobbers register b.

### 86. ChooseWildEncounter Validation
ValidateTempWildMonSpecies is called with `a` instead of `b`, not actually validating the species loaded from the encounter table.

### 87. RandomUnseenWildMon Time-of-Day
Always picks a morning Pokemon species because TimeOfDay offset is applied after the pointer calculation instead of before.

### 88. TryObjectEvent Arbitrary Code Execution
If IsInArray returns nc, `pop bc` is skipped and data at bc is executed as code.

### 89. ReadObjectEvents Buffer Overflow
Uses `NUM_OBJECTS` instead of `NUM_OBJECTS - 1`, overflowing into wObjectMasks.

### 90. ClearWRAM Only Bank 1
Loop condition `jr nc` should be `jr c`, so only WRAM bank 1 is cleared instead of banks 1-7.

### 91. BattleAnimCmd_ClearObjs Partial Clear
Uses hardcoded $A0 instead of `NUM_BATTLE_ANIM_STRUCTS * BATTLEANIMSTRUCT_LENGTH`, only clearing first 6.67 objects.

### 92. Options Menu Joypad State
Options menu doesn't clear joypad state on init, allowing all options to update simultaneously if buttons pressed on opening frame.

---

## Bugs Missing from Our Previous glitches_quirks.md

The following bugs from the official docs were NOT in our existing file:
- #9 (Defense-lowering after Substitute break)
- #11 (Disabled PP Up Struggle)
- #14-16 (Beat Up desync, single Pokemon, King's Rock)
- #17 (Present link damage)
- #18 (Return/Frustration zero damage)
- #20 (Switch freeze HP < 4)
- #21 (Stat boost moves skip after KO)
- #25 (Dude tutorial crash)
- #30 (Heavy Ball weight)
- #32 (PRZ/BRN on switch)
- #38 (Cautious AI residual)
- #40-43 (AI item bugs - only partially covered before)
- #46 (HELD_CATCH_CHANCE)
- #47 (Credits move menu)
- #48-52 (All overworld bugs)
- #53-64 (All graphics bugs)
- #65-69 (All audio bugs)
- #70-73 (All text bugs)
- #74-82 (All scripted event bugs)
- #83-92 (All internal engine bugs)

Our original file covered ~25 bugs. The official list has 92 documented bugs total.

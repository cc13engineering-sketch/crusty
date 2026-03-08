# Pokemon Crystal -- Comprehensive Scripting Commands Reference

Source: pokecrystal docs/ (event_commands.md, move_effect_commands.md, movement_commands.md, text_commands.md, music_commands.md, battle_anim_commands.md), macros/scripts/*.asm

---

## Event Script Commands (Overworld)

Defined in macros/scripts/events.asm. ~140 commands total.

### Flow Control
| Opcode | Command | Description |
|--------|---------|-------------|
| $00 | `scall script` | Call a script (same bank) |
| $01 | `farscall script` | Call a script (any bank) |
| $02 | `memcall script` | Call script at address in memory |
| $03 | `sjump script` | Jump to script (same bank) |
| $04 | `farsjump script` | Jump to script (any bank) |
| $05 | `memjump script` | Jump to script at address in memory |
| $06 | `ifequal byte, script` | Jump if wScriptVar == byte |
| $07 | `ifnotequal byte, script` | Jump if wScriptVar != byte |
| $08 | `iffalse script` | Jump if wScriptVar == 0 |
| $09 | `iftrue script` | Jump if wScriptVar != 0 |
| $0A | `ifgreater byte, script` | Jump if wScriptVar > byte |
| $0B | `ifless byte, script` | Jump if wScriptVar < byte |
| $0C | `jumpstd std_script` | Jump to standard script |
| $0D | `callstd std_script` | Call standard script |
| $0E | `callasm asm` | Call assembly routine |
| $0F | `special special_pointer` | Call special function |
| $10 | `memcallasm asm` | Call ASM at address in memory |

### Scene/Map Control
| Opcode | Command | Description |
|--------|---------|-------------|
| $11 | `checkmapscene map` | Check scene ID for a map |
| $12 | `setmapscene map, id` | Set scene ID for a map |
| $13 | `checkscene` | Check current map's scene |
| $14 | `setscene id` | Set current map's scene |

### Variables
| Opcode | Command | Description |
|--------|---------|-------------|
| $15 | `setval value` | wScriptVar = value |
| $16 | `addval value` | wScriptVar += value |
| $17 | `random value` | wScriptVar = random(0..value-1) |
| $18 | `checkver` | Check game version |
| $19 | `readmem addr` | wScriptVar = [addr] |
| $1A | `writemem addr` | [addr] = wScriptVar |
| $1B | `loadmem addr, value` | [addr] = value |
| $1C | `readvar variable` | Read game variable |
| $1D | `writevar variable` | Write game variable |
| $1E | `loadvar variable, value` | Set game variable to value |

### Items/Money/Coins
| Opcode | Command | Description |
|--------|---------|-------------|
| $1F | `giveitem item[, qty]` | Give item to player |
| $20 | `takeitem item[, qty]` | Remove item from player |
| $21 | `checkitem item` | Check if player has item |
| $22 | `givemoney acct, value` | Give money |
| $23 | `takemoney acct, value` | Take money |
| $24 | `checkmoney acct, value` | Check money |
| $25 | `givecoins value` | Give game corner coins |
| $26 | `takecoins value` | Take game corner coins |
| $27 | `checkcoins value` | Check coins |

### Phone
| Opcode | Command | Description |
|--------|---------|-------------|
| $28 | `addcellnum contact` | Add phone contact |
| $29 | `delcellnum contact` | Remove phone contact |
| $2A | `checkcellnum contact` | Check if contact exists |

### Pokemon
| Opcode | Command | Description |
|--------|---------|-------------|
| $2B | `checktime time` | Check time of day |
| $2C | `checkpoke species` | Check if species in party |
| $2D | `givepoke species, level, item` | Give Pokemon |
| $2E | `giveegg species, level` | Give Pokemon egg |
| $2F | `givepokemail ...` | Give Pokemon with mail |

### Event Flags
| Opcode | Command | Description |
|--------|---------|-------------|
| $30 | `checkevent flag` | Check event flag |
| $31 | `clearevent flag` | Clear event flag |
| $32 | `setevent flag` | Set event flag |
| $33 | `checkflag flag` | Check engine flag |
| $34 | `clearflag flag` | Clear engine flag |
| $35 | `setflag flag` | Set engine flag |

### Trainer Battles
| Opcode | Command | Description |
|--------|---------|-------------|
| $3C | `winlosstext win, loss` | Set battle result text |
| $3D | `scripttalkafter` | Start trainer script |
| $3E | `talkaftercancel` | Cancel talk-after |
| $3F | `talkaftercheck` | Check talk-after |
| $40 | `setlasttalked object` | Set last talked object |
| $43 | `trainertext id` | Show trainer text |
| $44 | `trainerflagaction flag` | Set trainer flag |

### Movement/Object Control
| Opcode | Command | Description |
|--------|---------|-------------|
| $53 | `applymovement object, data` | Apply movement script |
| $54 | `applymovementlasttalked data` | Move last talked object |
| $55 | `faceplayer` | Turn object to face player |
| $56 | `faceobject obj1, obj2` | Face one object toward another |
| $57 | `variablesprite sprite, id` | Change variable sprite |
| $58 | `disappear object` | Hide object |
| $59 | `appear object` | Show object |
| $5A | `follow object1, object2` | One object follows another |
| $5B | `stopfollow` | Stop following |
| $5C | `moveobject object, x, y` | Teleport object |
| $5D | `writeobjectxy object` | Write object coordinates |

### Text/Dialogue
| Opcode | Command | Description |
|--------|---------|-------------|
| $47 | `opentext` | Open text box |
| $48 | `closetext` | Close text box |
| $49 | `writetext text` | Write text to box |
| $4A | `farwritetext text` | Write text from other bank |
| $4B | `repeattext byte1, byte2` | Repeat previous text |
| $4C | `yesorno` | Yes/No prompt |
| $4D | `loadmenu header` | Load menu |
| $4E | `closewindow` | Close menu window |
| $4F | `jumptextfaceplayer text` | Face player and show text |
| $50 | `farjumptextfaceplayer text` | Far version |
| $51 | `jumptext text` | Just show text |
| $52 | `farjumptext text` | Far version |
| $67 | `waitbutton` | Wait for button press |
| $68 | `buttonsound` | Play button sound and wait |

### Map/Warp
| Opcode | Command | Description |
|--------|---------|-------------|
| $5E | `loademote emote` | Load emote bubble |
| $5F | `showemote emote, obj, time` | Show emote above object |
| $60 | `turnobject obj, dir` | Turn object to direction |
| $63 | `earthquake duration` | Screen shake |
| $69 | `warp map, x, y` | Warp to location |
| $6A | `warpfacing dir, map, x, y` | Warp and set facing |

### Music/Sound
| Opcode | Command | Description |
|--------|---------|-------------|
| $73 | `playsound sfx` | Play sound effect |
| $74 | `waitsfx` | Wait for SFX to finish |
| $75 | `warpsound` | Play warp sound |
| $76 | `cry species` | Play Pokemon cry |
| $77 | `playmapmusic` | Resume map music |
| $78 | `playmusic music_id` | Play music track |
| $79 | `musicfadeout music, frames` | Fade to new music |
| $7A | `playmapmusic` | Play current map music |

### Misc
| Opcode | Command | Description |
|--------|---------|-------------|
| $7B | `changeblock x, y, block` | Change map block |
| $7C | `reloadmappart` | Reload visible map |
| $7D | `reloadmap` | Full map reload |
| $8B | `end` | End script |

---

## Move Effect Commands (Battle)

Defined in macros/scripts/battle_commands.asm. These compose move effect scripts.

### Core Damage Flow
| Opcode | Command | Description |
|--------|---------|-------------|
| $01 | `checkturn` | Check if it's the user's turn |
| $02 | `checkobedience` | Check if Pokemon obeys |
| $03 | `usedmovetext` | Display "[Pokemon] used [Move]!" |
| $04 | `doturn` | Execute turn mechanics |
| $05 | `critical` | Calculate critical hit |
| $06 | `damagestats` | Get attack/defense stats for damage |
| $07 | `stab` | Calculate STAB and type effectiveness |
| $08 | `damagevariation` | Apply random damage variation (85-100%) |
| $09 | `checkhit` | Check accuracy/evasion |
| $0A | `lowersub` | Lower Substitute sprite |
| $0B | `hittargetnosub` | Hit target, ignore Substitute |
| $0C | `raisesub` | Raise Substitute sprite |
| $0D | `failuretext` | Display failure text |
| $0E | `applydamage` | Apply calculated damage |
| $0F | `criticaltext` | Display "Critical hit!" |
| $10 | `supereffectivetext` | Display effectiveness text |
| $11 | `checkfaint` | Check if target fainted |
| $12 | `buildopponentrage` | Build Rage counter |

### Status Effects
| Opcode | Command | Description |
|--------|---------|-------------|
| $13 | `poisontarget` | Attempt to poison |
| $14 | `sleeptarget` | Attempt to put to sleep |
| $15 | `draintarget` | Drain HP |
| $16 | `eatdream` | Dream Eater effect |
| $17 | `burntarget` | Attempt to burn |
| $18 | `freezetarget` | Attempt to freeze |
| $19 | `paralyzetarget` | Attempt to paralyze |
| $1A | `selfdestruct` | Self-destruct (halve defense) |
| $1B | `mirrormove` | Use opponent's last move |

### Stat Changes
| Opcode | Command | Description |
|--------|---------|-------------|
| $1C | `statup` | Raise user's stat |
| $1D | `statdown` | Lower target's stat |
| $1E | `payday` | Scatter coins |
| $1F | `conversion` | Conversion type change |
| $20 | `resetstats` | Haze - reset all stat changes |
| $21 | `storeenergy` | Bide - store energy |
| $22 | `unleashenergy` | Bide - release damage |
| $23 | `forceswitch` | Whirlwind/Roar |
| $2E | `effectchance` | Check secondary effect chance |

### Specific Move Effects
| Opcode | Command | Description |
|--------|---------|-------------|
| $29 | `recoil` | Apply recoil damage |
| $2A | `mist` | Set Mist screen |
| $2B | `focusenergy` | Set Focus Energy |
| $2C | `confuse` | Attempt to confuse |
| $33 | `substitute` | Create Substitute |
| $34 | `copydefense` | Psych Up |
| $35 | `leechseed` | Plant Leech Seed |
| $36 | `disable` | Disable a move |
| $37 | `counter` | Counter (reflect physical) |
| $38 | `encore` | Lock opponent into move |
| $39 | `painsplit` | Split HP evenly |
| $3A | `snore` | Snore (sleep move) |
| $3B | `conversion2` | Change type to resist |
| $3C | `lockon` | Guarantee next hit |
| $3D | `sketch` | Permanently copy move |
| $3F | `sleeptalk` | Use random move while asleep |
| $40 | `destinybond` | Faint attacker if user faints |
| $41 | `spite` | Reduce PP of last move used |
| $42 | `falseswipenohit` | Leave target at 1 HP |
| $43 | `healbell` | Cure party status |
| $47 | `protect` | Protect/Detect |
| $48 | `spikes` | Set entry hazard |
| $49 | `foresight` | Remove type immunity |
| $4A | `perishsong` | 3-turn faint countdown |
| $4B | `sandstorm` | Start sandstorm |
| $4C | `endure` | Survive with 1 HP |
| $4D | `rolloutpower` | Rollout/Ice Ball power |
| $4E | `attract` | Infatuate opposite gender |
| $4F | `happinesspower` | Return power from happiness |
| $50 | `present` | Random damage/heal |
| $51 | `frustrationpower` | Frustration power |
| $52 | `safeguard` | Prevent status |
| $53 | `magnitudecalc` | Random earthquake power |
| $54 | `batonpass` | Switch and pass stats |
| $55 | `pursuit` | Hit on switch |
| $56 | `mirrorcoat` | Reflect special damage |
| $59 | `futuresight` | Delayed attack |
| $5A | `beatup` | Party multi-hit |
| $5E | `thief` | Steal held item |
| $5F | `arenatrap` | Mean Look/Spider Web |
| $60 | `nightmare` | Damage sleeping target each turn |
| $61 | `curse` | Ghost/non-Ghost curse |
| $64 | `rain` | Start rain |
| $65 | `sunny` | Start sun |
| $66 | `bellydrum` | Max Attack at HP cost |
| $67 | `psychup` | Copy stat changes |
| $68 | `rage` | Rage - boost Attack on hit |

### Stat Shortcuts
| Command | Description |
|---------|-------------|
| `attackup` / `attackup2` | Raise Attack +1/+2 |
| `defenseup` / `defenseup2` | Raise Defense +1/+2 |
| `speedup` / `speedup2` | Raise Speed +1/+2 |
| `specialattackup` / `specialattackup2` | Raise SpAtk +1/+2 |
| `specialdefenseup` / `specialdefenseup2` | Raise SpDef +1/+2 |
| `accuracyup` / `evasionup` | Raise Accuracy/Evasion +1 |
| `attackdown` / `defensedown` | Lower Attack/Defense -1 |
| `speeddown` / `specialdefensedown` | Lower Speed/SpDef -1 |
| `accuracydown` / `evasiondown` | Lower Accuracy/Evasion -1 |
| `allstatsup` | AncientPower - all stats +1 |
| `hittarget` | Apply hit to target |
| `damagecalc` | Full damage calculation |
| `statupmessage` / `statdownmessage` | Display stat change text |
| `endmove` | End move script |

---

## Movement Commands (NPC/Player)

Defined in macros/scripts/movement.asm.

| Opcode | Command | Description |
|--------|---------|-------------|
| $00-$03 | `turn_head DIR` | Turn head (DOWN/UP/LEFT/RIGHT) |
| $04-$07 | `turn_step DIR` | Turn and step |
| $08-$0B | `slow_step DIR` | Slow step |
| $0C-$0F | `step DIR` | Normal step |
| $10-$13 | `big_step DIR` | Big (fast) step |
| $14-$17 | `slow_slide_step DIR` | Slow sliding step |
| $18-$1B | `slide_step DIR` | Sliding step (ice) |
| $1C-$1F | `fast_slide_step DIR` | Fast sliding step |
| $20-$23 | `turn_away DIR` | Turn away from direction |
| $24-$27 | `turn_in DIR` | Turn in place |
| $28-$2B | `turn_waterfall DIR` | Waterfall movement |
| $2C-$2F | `slow_jump_step DIR` | Slow jump (ledge) |
| $30-$33 | `jump_step DIR` | Normal jump |
| $34-$37 | `fast_jump_step DIR` | Fast jump |
| $38 | `remove_sliding` | Stop sliding |
| $39 | `set_sliding` | Start sliding |
| $3A | `remove_fixed_facing` | Allow facing changes |
| $3B | `fix_facing` | Lock facing direction |
| $3C | `show_object` | Show sprite |
| $3D | `hide_object` | Hide sprite |
| $3E-$46 | `step_sleep N` | Pause for N frames |
| $47 | `step_end` | End movement script |
| $48 | `step_48 param` | Unknown |
| $49 | `remove_object` | Remove from map |
| $4A | `step_loop` | Loop movement |
| $4C | `teleport_from` | Teleport departure anim |
| $4D | `teleport_to` | Teleport arrival anim |
| $4E | `skyfall` | Fall from sky |
| $4F | `step_dig N` | Dig animation |
| $50 | `step_bump` | Bump animation |
| $51 | `fish_got_bite` | Fishing bite animation |
| $52 | `fish_cast_rod` | Cast rod animation |
| $53 | `hide_emote` | Hide emote bubble |
| $54 | `show_emote` | Show emote bubble |
| $55 | `step_shake disp` | Shake/vibrate |
| $56 | `tree_shake` | Headbutt tree shake |
| $57 | `rock_smash N` | Rock Smash animation |
| $58 | `return_dig N` | Return from Dig |
| $59 | `skyfall_top` | Skyfall from top |

---

## Text Commands

Defined in macros/scripts/text.asm.

| Opcode | Command | Description |
|--------|---------|-------------|
| $00 | `text_start` | Start writing text until "@" |
| $01 | `text_ram addr` | Write text from RAM address |
| $02 | `text_bcd addr, flags` | Write BCD number |
| $03 | `text_move addr` | Move cursor to tile |
| $04 | `text_box addr, h, w` | Draw a box |
| $05 | `text_low` | Write at (1,16) |
| $06 | `text_promptbutton` | Wait for button, show arrow |
| $07 | `text_scroll` | Push text up two lines |
| $08 | `text_asm` | Start interpreting ASM |
| $09 | `text_decimal addr, bytes, digits` | Print number |
| $0A | `text_pause` | Pause 30 frames |
| $0B | `sound_dex_fanfare_50_79` | Play SFX |
| $0C | `text_dots n` | Print n "..."s with pause |
| $0D | `text_waitbutton` | Wait, no arrow |
| $0E | `sound_dex_fanfare_20_49` | Play SFX |
| $0F | `sound_item` | Play item get SFX |
| $10 | `sound_caught_mon` | Play catch SFX |
| $11 | `sound_dex_fanfare_80_109` | Play SFX |
| $12 | `sound_fanfare` | Play fanfare SFX |
| $13 | `sound_slot_machine_start` | Play slot SFX |
| $14 | `text_buffer id` | Write from buffer (0-6) |
| $15 | `text_today` | Print weekday |
| $16 | `text_far addr` | Write text from other bank |
| $50 | `text_end` | End text processing |

### Control Characters (inline)
| Code | Macro | Description |
|------|-------|-------------|
| $4E | `next text` | Move one line down |
| $4F | `line text` | Start bottom line |
| $50 | `page text` | New Pokedex page |
| $51 | `para text` | New paragraph |
| $55 | `cont text` | Scroll to next line |
| $57 | `done` | End text box |
| $58 | `prompt` | Prompt to close |

### Text Buffers (text_buffer IDs)
| ID | Address | Content |
|----|---------|---------|
| 0 | wStringBuffer3 | General purpose |
| 1 | wStringBuffer4 | General purpose |
| 2 | wStringBuffer5 | General purpose |
| 3 | wStringBuffer2 | General purpose |
| 4 | wStringBuffer1 | General purpose |
| 5 | wEnemyMonNickname | Enemy Pokemon name |
| 6 | wBattleMonNickname | Player Pokemon name |

---

## Music Commands (Audio Engine)

Defined in macros/scripts/audio.asm.

### Song Structure
| Command | Description |
|---------|-------------|
| `channel_count n` | Set number of channels (1-4) |
| `channel index, addr` | Assign channel data pointer |

### Notes (Channels 1-3)
| Command | Description |
|---------|-------------|
| `note pitch, length` | Play note (length 1-16 ticks) |
| `rest length` | Rest (1-16 ticks) |
| `octave n` | Set octave (1-8), opcodes $D0-$D7 |
| `note_type len, vol, fade` | Set note length, volume, fade |
| `drum_speed length` | Set drum note length (ch4) |
| `drum_note inst, length` | Play drum (ch4, instrument 1-12) |

### Sound Effects (Channels 5-8)
| Command | Description |
|---------|-------------|
| `square_note len, vol, fade, freq` | Square wave SFX note (ch5-7) |
| `noise_note len, vol, fade, freq` | Noise SFX note (ch8) |

### Modifiers
| Opcode | Command | Description |
|--------|---------|-------------|
| $D9 | `transpose oct, pitch` | Transpose notes |
| $DA | `tempo value` | Set tempo (BPM = 19200/value) |
| $DB | `duty_cycle dc` | Set duty cycle (0-3) |
| $DC | `volume_envelope vol, fade` | Set volume envelope |
| $DD | `pitch_sweep len, change` | Pitch sweep (ch5 only) |
| $DE | `duty_cycle_pattern a,b,c,d` | PWM pattern (ch5-6) |
| $DF | `toggle_sfx` | Toggle song/SFX mode |
| $E0 | `pitch_slide dur, oct, pitch` | Bend pitch of next note |
| $E1 | `vibrato delay, extent, rate` | Apply vibrato |
| $E3 | `toggle_noise id` | Set drum kit (0-5) |
| $E4 | `force_stereo_panning l, r` | Force stereo output |
| $E5 | `volume left, right` | Set master volume (0-7) |
| $E6 | `pitch_offset value` | Adjust all pitches |
| $E9 | `tempo_relative value` | Adjust tempo relatively |
| $EA | `restart_channel addr` | Restart channel |
| $EB | `new_song id` | Start new song |
| $EC | `sfx_priority_on` | SFX overrides music |
| $ED | `sfx_priority_off` | Music overrides SFX |
| $EF | `stereo_panning l, r` | Stereo if user enabled |
| $F0 | `sfx_toggle_noise id` | Set SFX drum kit (ch8) |
| $FA | `set_condition cond` | Set jump condition |
| $FB | `sound_jump_if cond, addr` | Conditional jump |
| $FC | `sound_jump addr` | Unconditional jump |
| $FD | `sound_loop count, addr` | Loop (0 = infinite) |
| $FE | `sound_call addr` | Call subroutine |
| $FF | `sound_ret` | Return from call / end |

---

## Map Script Structure

### Map Scripts Header
```
MapName_MapScripts:
    def_scene_scripts          ; Scene-triggered scripts
    scene_script script, SCENE_NAME

    def_callbacks              ; Map load callbacks
    callback TYPE, script
```

### Callback Types
- `MAPCALLBACK_NEWMAP` -- Runs when entering map for first time
- `MAPCALLBACK_TILES` -- Modify tiles on load
- `MAPCALLBACK_OBJECTS` -- Modify objects on load
- `MAPCALLBACK_SPRITES` -- Modify sprites on load
- `MAPCALLBACK_CMDQUEUE` -- Set up command queue (boulders)

### Map Events
```
MapName_MapEvents:
    db 0, 0                    ; filler
    def_warp_events
    warp_event x, y, MAP, warp_id
    def_coord_events
    coord_event x, y, scene_id, script
    def_bg_events
    bg_event x, y, TYPE, script
    def_object_events
    object_event x, y, sprite, movement, rx, ry, h1, h2, palette, type, range, script, flag
```

### BG Event Types
- `BGEVENT_READ` -- Read sign/object facing any direction
- `BGEVENT_UP/DOWN/LEFT/RIGHT` -- Direction-specific read
- `BGEVENT_IFSET/IFNOTSET` -- Conditional on event flag
- `BGEVENT_ITEM` -- Hidden item (`hiddenitem item, flag`)
- `BGEVENT_COPY` -- Copy tile data

### Object Types
- `OBJECTTYPE_SCRIPT` -- NPC with script
- `OBJECTTYPE_ITEMBALL` -- Item ball (`itemball item[, qty]`)
- `OBJECTTYPE_TRAINER` -- Trainer (`trainer group, id, flag, seen, beaten, loss, script`)

### Movement Types (for object_event)
- `SPRITEMOVEDATA_STILL` -- Stationary
- `SPRITEMOVEDATA_WANDER` -- Random walk within radius
- `SPRITEMOVEDATA_SPINRANDOM_SLOW/FAST` -- Random facing
- `SPRITEMOVEDATA_WALK_UP_DOWN` -- Pace vertically
- `SPRITEMOVEDATA_WALK_LEFT_RIGHT` -- Pace horizontally
- `SPRITEMOVEDATA_STANDING_UP/DOWN/LEFT/RIGHT` -- Fixed facing
- `SPRITEMOVEDATA_POKEMON` -- Pokemon movement
- `SPRITEMOVEDATA_SUDOWOODO` -- Sudowoodo blocking
- `SPRITEMOVEDATA_SMASHABLE_ROCK` -- Rock Smash boulder
- `SPRITEMOVEDATA_STRENGTH_BOULDER` -- Strength boulder
- `SPRITEMOVEDATA_SPINCOUNTERCLOCKWISE/CLOCKWISE` -- Spin
- `SPRITEMOVEDATA_SWIM_WANDER` -- Water wandering
- `SPRITEMOVEDATA_BIGDOLL/BIGDOLLSYM/BIGDOLLASYM` -- Decorations

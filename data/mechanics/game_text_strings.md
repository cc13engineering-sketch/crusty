# Pokemon Crystal - Game Text Strings

All battle messages, common text, and standard text from pokecrystal data/text/.

Source: `data/text/battle.asm`, `data/text/common_1.asm`, `data/text/common_2.asm`, `data/text/common_3.asm`, `data/text/std_text.asm`, `data/text/name_input_chars.asm`, `data/text/mail_input_chars.asm`, `data/text/unused_*.asm`

---

## Battle Text (data/text/battle.asm)

### Wild Encounter Messages
| Label | Text |
|-------|------|
| WildPokemonAppeared | "Wild [MON] appeared!" |
| HookedPokemonAttacked | "The hooked [MON] attacked!" |
| PokemonFellFromTree | "[MON] fell out of the tree!" |
| WildCelebiAppeared | "Wild [MON] appeared!" |
| WildFled | "Wild [MON] fled!" |
| EnemyFled | "Enemy [MON] fled!" |

### Trainer Battle Messages
| Label | Text |
|-------|------|
| WantsToBattle | "[ENEMY] wants to battle!" |
| EnemyIsAboutToUse | "[ENEMY] is about to use [MON]. Will [PLAYER] change #MON?" |
| EnemySentOut | "[ENEMY] sent out [MON]!" |
| EnemyMonFainted | "Enemy [MON] fainted!" |
| EnemyWasDefeated | "[ENEMY] was defeated!" |
| GotMoneyForWinning | "[PLAYER] got $[amount] for winning!" |
| TiedAgainst | "Tied against [ENEMY]!" |
| LostAgainst | "Lost against [ENEMY]!" |
| SentSomeToMom | "[PLAYER] got $[amount] for winning! Sent some to MOM!" |
| SentHalfToMom | "Sent half to MOM!" |
| SentAllToMom | "Sent all to MOM!" |

### Player Pokemon Messages
| Label | Text |
|-------|------|
| MonFainted | "[MON] fainted!" |
| UseNextMon | "Use next #MON?" |
| GoMon | "Go! [MON]!" |
| DoItMon | "Do it! [MON]!" |
| GoForItMon | "Go for it, [MON]!" |
| YourFoesWeakGetmMon | "Your foe's weak! Get'm, [MON]!" |
| ThatsEnoughComeBack | "[MON] that's enough! Come back!" |
| OKComeBack | "[MON] OK! Come back!" |
| GoodComeBack | "[MON] good! Come back!" |
| ComeBack | "[MON] come back!" |
| MonIsAlreadyOut | "[MON] is already out." |
| MonCantBeRecalled | "[MON] can't be recalled!" |
| MonHasNoMovesLeft | "[MON] has no moves left!" |
| TheresNoWillToBattle | "There's no will to battle!" |
| AnEGGCantBattle | "An EGG can't battle!" |

### Move Usage Messages
| Label | Text |
|-------|------|
| UsedMove | "[USER] used [MOVE]!" |
| UsedInstead | "instead, [MOVE]!" |
| TheresNoPPLeftForThisMove | "There's no PP left for this move!" |
| TheMoveIsDisabled | "The move is DISABLED!" |
| NoPPLeft | "But no PP is left for the move!" |
| HasNoPPLeft | "[USER] has no PP left for [MOVE]!" |

### Effectiveness Messages
| Label | Text |
|-------|------|
| CriticalHit | "A critical hit!" |
| OneHitKO | "It's a one-hit KO!" |
| SuperEffective | "It's super-effective!" |
| NotVeryEffective | "It's not very effective..." |
| DoesntAffect | "It doesn't affect [TARGET]!" |
| Unaffected | "[TARGET]'s unaffected!" |
| DidntAffect | "It didn't affect [TARGET]!" |
| NothingHappened | "But nothing happened." |
| ButItFailed | "But it failed!" |
| ItFailed | "It failed!" |

### Status Condition Messages
| Label | Text |
|-------|------|
| FellAsleep | "[TARGET] fell asleep!" |
| AlreadyAsleep | "[TARGET]'s already asleep!" |
| FastAsleep | "[USER] is fast asleep!" |
| WokeUp | "[USER] woke up!" |
| WasPoisoned | "[TARGET] was poisoned!" |
| BadlyPoisoned | "[TARGET]'s badly poisoned!" |
| AlreadyPoisoned | "[TARGET]'s already poisoned!" |
| WasBurned | "[TARGET] was burned!" |
| WasFrozen | "[TARGET] was frozen solid!" |
| FrozenSolid | "[USER] is frozen solid!" |
| DefrostedOpponent | "[TARGET] was defrosted!" |
| WasDefrosted | "[USER] was defrosted!" |
| Paralyzed | "[TARGET]'s paralyzed! Maybe it can't attack!" |
| FullyParalyzed | "[USER]'s fully paralyzed!" |
| AlreadyParalyzed | "[TARGET]'s already paralyzed!" |
| BecameConfused | "[TARGET] became confused!" |
| IsConfused | "[USER] is confused!" |
| HurtItself | "It hurt itself in its confusion!" |
| ConfusedNoMore | "[USER]'s confused no more!" |
| AlreadyConfused | "[TARGET]'s already confused!" |
| ItemHealedConfusion | "A [ITEM] rid [TARGET] of its confusion." |
| Flinched | "[USER] flinched!" |

### Residual Damage Messages
| Label | Text |
|-------|------|
| HurtByPoison | "[USER] is hurt by poison!" |
| HurtByBurn | "[USER]'s hurt by its burn!" |
| LeechSeedSaps | "LEECH SEED saps [USER]!" |
| HasANightmare | "[USER] has a NIGHTMARE!" |
| HurtByCurse | "[USER]'s hurt by the CURSE!" |
| SandstormHits | "The SANDSTORM hits [USER]!" |
| PerishCount | "[USER]'s PERISH count is [N]!" |
| HurtBySpikes | "[USER]'s hurt by SPIKES!" |
| Recoil | "[USER]'s hit with recoil!" |

### Weather Messages
| Label | Text |
|-------|------|
| Downpour | "A downpour started!" |
| SunGotBright | "The sunlight got bright!" |
| SandstormBrewed | "A SANDSTORM brewed!" |
| RainContinuesToFall | "Rain continues to fall." |
| TheSunlightIsStrong | "The sunlight is strong." |
| TheSandstormRages | "The SANDSTORM rages." |
| TheRainStopped | "The rain stopped." |
| TheSunlightFaded | "The sunlight faded." |
| TheSandstormSubsided | "The SANDSTORM subsided." |

### Stat Change Messages
| Label | Text |
|-------|------|
| StatWentUp | "[USER]'s [STAT] went up!" |
| StatWentWayUp | "[USER]'s [STAT] went way up!" |
| StatFell | "[TARGET]'s [STAT] fell!" |
| StatSharplyFell | "[TARGET]'s [STAT] sharply fell!" |
| WontRiseAnymore | "[USER]'s [STAT] won't rise anymore!" |
| WontDropAnymore | "[TARGET]'s [STAT] won't drop anymore!" |

### Specific Move Effect Messages
| Label | Text |
|-------|------|
| MadeSubstitute | "[USER] made a SUBSTITUTE!" |
| HasSubstitute | "[USER] has a SUBSTITUTE!" |
| TooWeakSub | "Too weak to make a SUBSTITUTE!" |
| SubTookDamage | "The SUBSTITUTE took damage for [TARGET]!" |
| SubFaded | "[TARGET]'s SUBSTITUTE faded!" |
| WasSeeded | "[TARGET] was seeded!" |
| Evaded | "[TARGET] evaded the attack!" |
| WasDisabled | "[TARGET]'s [MOVE] was DISABLED!" |
| DisabledNoMore | "[USER]'s disabled no more!" |
| MimicLearnedMove | "[USER] learned [MOVE]!" |
| Sketched | "[USER] SKETCHED [MOVE]!" |
| Transformed | "[USER] TRANSFORMED into [MON]!" |
| TransformedType | "[USER] transformed into the [TYPE]-type!" |
| EliminatedStats | "All stat changes were eliminated!" |
| CopiedStats | "[USER] copied the stat changes of [TARGET]!" |
| Mist | "[USER]'s shrouded in MIST!" |
| ProtectedByMist | "[TARGET]'s protected by MIST." |
| GettingPumped | "[USER]'s getting pumped!" |
| LightScreenEffect | "[USER]'s SPCL.DEF rose!" |
| ReflectEffect | "[USER]'s DEFENSE rose!" |
| CoinsScattered | "Coins scattered everywhere!" |
| PlayerPickedUpPayDayMoney | "[PLAYER] picked up $[amount]!" |
| SuckedHealth | "Sucked health from [TARGET]!" |
| DreamEaten | "[TARGET]'s dream was eaten!" |
| StoringEnergy | "[USER] is storing energy!" |
| UnleashedEnergy | "[USER] unleashed energy!" |
| RageBuildingText | "[USER]'s RAGE is building!" |
| BellyDrum | "[USER] cut its HP and maximized ATTACK!" |
| SharedPain | "The battlers shared pain!" |
| PutACurse | "[USER] cut its own HP and put a CURSE on [TARGET]!" |
| CantEscapeNow | "[TARGET] can't escape now!" |
| StartedNightmare | "[TARGET] started to have a NIGHTMARE!" |
| FellInLove | "[TARGET] fell in love!" |
| InLoveWith | "[USER] is in love with [TARGET]!" |
| Infatuation | "[USER]'s infatuation kept it from attacking!" |
| ProtectedItself | "[USER] PROTECTED itself!" |
| ProtectingItself | "[TARGET]'s PROTECTING itself!" |
| Spikes | "SPIKES scattered all around [TARGET]!" |
| Identified | "[USER] identified [TARGET]!" |
| StartPerish | "Both #MON will faint in 3 turns!" |
| BracedItself | "[USER] braced itself!" |
| CoveredByVeil | "[USER]'s covered by a veil!" |
| SafeguardProtect | "[TARGET] is protected by SAFEGUARD!" |
| SafeguardFaded | "[USER]'s SAFEGUARD faded!" |
| LightScreenFell | "[SIDE] #MON's LIGHT SCREEN fell!" |
| ReflectFaded | "[SIDE] #MON's REFLECT faded!" |
| Magnitude | "Magnitude [N]!" |
| DestinyBondEffect | "[USER]'s trying to take its opponent with it!" |
| TookDownWithIt | "[TARGET] took down with it, [USER]!" |
| SpiteEffect | "[TARGET]'s [MOVE] was reduced by [N]!" |
| BellChimed | "A bell chimed!" |
| GotAnEncore | "[TARGET] got an ENCORE!" |
| EncoreEnded | "[TARGET]'s ENCORE ended!" |
| TookAim | "[USER] took aim!" |
| ForesawAttack | "[USER] foresaw an attack!" |
| TargetWasHitByFutureSight | "[TARGET] was hit by FUTURE SIGHT!" |
| BeatUpAttack | "[MON]'s attack!" |
| PresentFailed | "[TARGET] refused the gift!" |
| Stole | "[USER] stole [ITEM] from its foe!" |
| MirrorMoveFailed | "The MIRROR MOVE failed!" |
| ProtectedBy | "[TARGET]'s protected by [ITEM]!" |

### Trapping Move Messages
| Label | Text |
|-------|------|
| UsedBind | "[USER] used BIND on [TARGET]!" |
| WhirlpoolTrap | "[TARGET] was trapped!" |
| FireSpinTrap | "[TARGET] was trapped!" |
| WrappedBy | "[TARGET] was WRAPPED by [USER]!" |
| ClampedBy | "[TARGET] was CLAMPED by [USER]!" |
| HurtByStringBuffer1 | "[USER]'s hurt by [TRAP]!" |
| WasReleasedFromStringBuffer1 | "[USER] was released from [TRAP]!" |

### Endure/Held Item Messages
| Label | Text |
|-------|------|
| HungOn | "[TARGET] hung on with [ITEM]!" |
| Endured | "[TARGET] ENDURED the hit!" |
| RecoveredWithItem | "[TARGET] recovered with [ITEM]." |
| RecoveredPPUsing | "[USER] recovered PP using [ITEM]." |
| UsersItemActivated | "[USER]'s [ITEM] activated!" |
| RecoveredUsing | "[TARGET] recovered using a [ITEM]!" |

### Escape Messages
| Label | Text |
|-------|------|
| GotAwaySafely | "Got away safely!" |
| CantEscape | "Can't escape!" |
| TheresNoEscapeFromTrainerBattle | "No! There's no running from a trainer battle!" |
| FledUsingItem | "[USER] fled using a [ITEM]!" |
| FledFromBattle | "[USER] fled from battle!" |
| FledInFear | "[TARGET] fled in fear!" |
| BlownAway | "[TARGET] was blown away!" |
| DraggedOut | "[USER] was dragged out!" |
| ReleasedBy | "[USER] was released by [TARGET]!" |
| ShedLeechSeed | "[USER] shed LEECH SEED!" |
| BlewSpikes | "[USER] blew away SPIKES!" |

### Charge/Two-Turn Move Messages
| Label | Text |
|-------|------|
| MadeWhirlwind | "[USER] made a whirlwind!" |
| TookSunlight | "[USER] took in sunlight!" |
| LoweredHead | "[USER] lowered its head!" |
| Glowing | "[USER] is glowing!" |
| Flew | "[USER] flew up high!" |
| Dug | "[USER] dug a hole!" |
| MustRecharge | "[USER] must recharge!" |
| Crashed | "[USER] kept going and crashed!" |
| AttackMissed | "[USER]'s attack missed!" |

### Sleep-Related Messages
| Label | Text |
|-------|------|
| WentToSleep | "[USER] went to sleep!" |
| Rested | "[USER] fell asleep and became healthy!" |
| RegainedHealth | "[USER] regained health!" |

### Multi-Hit Messages
| Label | Text |
|-------|------|
| PlayerHitTimes | "Hit [N] times!" |
| EnemyHitTimes | "Hit [N] times!" |

### Disobedience Messages
| Label | Text |
|-------|------|
| LoafingAround | "[MON] is loafing around." |
| BeganToNap | "[MON] began to nap!" |
| WontObey | "[MON] won't obey!" |
| TurnedAway | "[MON] turned away!" |
| IgnoredOrders | "[MON] ignored orders!" |
| IgnoredSleeping | "[MON] ignored orders...sleeping!" |
| IgnoredOrders2 | "[USER] ignored orders!" |

### Wild Bait/Rock Messages
| Label | Text |
|-------|------|
| WildMonIsEating | "Wild [MON] is eating!" |
| WildMonIsAngry | "Wild [MON] is angry!" |

### Catch/Ball Messages
| Label | Text |
|-------|------|
| BallBrokeFree | "Oh no! The #MON broke free!" |
| BallAppearedCaught | "Aww! It appeared to be caught!" |
| BallAlmostHadIt | "Aargh! Almost had it!" |
| BallSoClose | "Shoot! It was so close too!" |
| BallCaught | "Gotcha! [MON] was caught!" |
| BallDodged | "It dodged the thrown BALL! This #MON can't be caught!" |
| BallMissed | "You missed the #MON!" |
| BallBlocked | "The trainer blocked the BALL!" |
| BallDontBeAThief | "Don't be a thief!" |
| BallBoxFull | "The #MON BOX is full. That can't be used now." |
| BallSentToPC | "[MON] was sent to BILL's PC." |
| NewDexData | "[MON]'s data was newly added to the #DEX." |

### Item Use Messages
| Label | Text |
|-------|------|
| ItemsCantBeUsedHere | "Items can't be used here." |
| HPIsFull | "[USER]'s HP is full!" |

### EXP/Level Messages
| Label | Text |
|-------|------|
| GainedExp | "[MON] gained [N] EXP. Points!" |
| BoostedExpPoints | "[MON] gained a boosted [N] EXP. Points!" |
| GrewToLevel | "[MON] grew to level [N]!" |

### Link Battle Messages
| Label | Text |
|-------|------|
| LinkErrorBattleCanceled | "Link error... The battle has been canceled..." |

---

## Common Text 1 (data/text/common_1.asm)

### Berry/Fruit Tree Text
| Label | Text |
|-------|------|
| FruitBearingTree | "It's a fruit-bearing tree." |
| HeyItsFruit | "Hey! It's [BERRY]!" |
| ObtainedFruit | "Obtained [BERRY]!" |
| FruitPackIsFull | "But the PACK is full..." |
| NothingHere | "There's nothing here..." |
| WhichApricorn | "Which APRICORN should I use?" |
| HowManyShouldIMake | "How many should I make?" |

### Healing Text
| Label | Text |
|-------|------|
| RecoveredSomeHP | "[MON] recovered [N]HP!" |
| CuredOfPoison | "[MON]'s cured of poison." |
| RidOfParalysis | "[MON]'s rid of paralysis." |
| BurnWasHealed | "[MON]'s burn was healed." |
| WasDefrosted | "[MON] was defrosted." |
| WokeUp | "[MON] woke up." |
| HealthReturned | "[MON]'s health returned." |
| Revitalized | "[MON] is revitalized." |
| GrewToLevel | "[MON] grew to level [N]!" |
| CameToItsSenses | "[MON] came to its senses." |

### Oak's Time Setup Text
| Label | Text |
|-------|------|
| OakTimeWokeUp | "......... Zzz... Hm? Wha...? You woke me up!" |
| OakTimeWhatTimeIsIt | "What time is it?" |
| OakTimeHowManyMinutes | "How many minutes?" |
| OakTimeOverslept | "! I overslept!" |
| OakTimeYikes | "! Yikes! I overslept!" |
| OakTimeSoDark | "! No wonder it's so dark!" |
| OakTimeWhatDayIsIt | "What day is it?" |

### Card Folder / Passcode Text
| Label | Text |
|-------|------|
| EnterNewPasscode | "Please enter any four-digit number." |
| ConfirmPasscode | "Enter the same number to confirm." |
| PasscodesNotSame | "That's not the same number." |
| PasscodeSet | "Your PASSCODE has been set." |
| FourZerosInvalid | "0000 is invalid!" |
| EnterPasscode | "Enter the CARD FOLDER PASSCODE." |
| IncorrectPasscode | "Incorrect PASSCODE!" |
| CardFolderOpen | "CARD FOLDER open." |

### Room Decoration Text
| Label | Text |
|-------|------|
| WhichSidePutOn | "Which side do you want to put it on?" |
| WhichSidePutAway | "Which side do you want to put away?" |
| PutAwayTheDeco | "Put away the [DECO]." |
| NothingToPutAway | "There's nothing to put away." |
| SetUpTheDeco | "Set up the [DECO]." |
| AlreadySetUp | "That's already set up." |
| LookTownMap | "It's the TOWN MAP." |
| LookPikachuPoster | "It's a poster of a cute PIKACHU." |
| LookClefairyPoster | "It's a poster of a cute CLEFAIRY." |
| LookJigglypuffPoster | "It's a poster of a cute JIGGLYPUFF." |
| LookGiantDeco | "A giant doll! It's fluffy and cuddly." |

### Mom's Shopping Messages
| Label | Text |
|-------|------|
| MomHiHowAreYou | "Hi, [PLAYER]! How are you?" |
| MomFoundAnItem | "I found a useful item shopping, so" |
| MomBoughtWithYourMoney | "I bought it with your money. Sorry!" |
| MomItsInPC | "It's in your PC. You'll like it!" |
| MomFoundADoll | "While shopping today, I saw this adorable doll, so" |
| MomItsInYourRoom | "It's in your room. You'll love it!" |

### Trading Text
| Label | Text |
|-------|------|
| MonWasSentTo | "[MON] was sent to [TRAINER]." |
| BidsFarewellTo | "[TRAINER] bids farewell to [MON]." |
| TakeGoodCareOfMon | "Take good care of [MON]." |
| ForYourMonSends | "For [PLAYER]'s [MON]," |
| OTSends | "[TRADER] sends [MON]." |
| WillTrade | "[TRADER] will trade [MON]" |

### Radio Station Text (Oak's Pokemon Talk)
16 adverbs and 16 adjectives are randomly combined to describe Pokemon sightings on the radio:

**Adverbs:** sweet and adorably, wiggly and slickly, aptly named and, undeniably kind of, so so unbearably, wow impressively, almost poisonously, ooh so sensually, so mischievously, so very topically, sure addictively, looks in water is, evolution must be, provocatively, so flipped out and, heart-meltingly

**Adjectives:** cute, weird, pleasant, bold sort of, frightening, suave & debonair!, powerful, exciting, groovy!, inspiring, friendly, hot hot hot!, stimulating, guarded, lovely, speedy

### Radio Station Text (Pokemon Music Channel)
| Label | Text |
|-------|------|
| BenIntro | "BEN: #MON MUSIC CHANNEL! It's me, DJ BEN!" |
| FernIntro | "FERN: #MUSIC! With DJ FERN!" |
| BenFernJam | "Today's [DAY], so let us jam to #MON March!" |
| BenFernChill | "Today's [DAY], so chill out to #MON Lullaby!" |

### Radio Station Text (Lucky Number Show)
| Label | Text |
|-------|------|
| LC_Intro | "REED: Yeehaw! How y'all doin' now?" |
| LC_Pitch | "Whether you're up or way down low, don't you miss the LUCKY NUMBER SHOW!" |
| LC_Number | "This week's Lucky Number is [NUMBER]!" |
| LC_Repeat | "I'll repeat that! Match it and go to the RADIO TOWER!" |
| LC_Drag | "...Repeating myself gets to be a drag..." |

### Radio Station Text (Places and People)
| Label | Text |
|-------|------|
| PnP_Intro | "PLACES AND PEOPLE! Brought to you by me, DJ LILY!" |

16 personality descriptions: cute, sort of lazy, always happy, quite noisy, precocious, somewhat bold, too picky!, sort of OK, just so-so, actually great, just my type, so cool no?, inspiring!, kind of weird, right for me?, definitely odd!

### Radio Station Text (Team Rocket Takeover)
| Label | Text |
|-------|------|
| RocketRadio | "...Ahem, we are TEAM ROCKET! After three years of preparation, we have risen again from the ashes! GIOVANNI! Can you hear? We did it! Where is our boss? Is he listening?" |

### Radio Station Text (Buena's Password)
| Label | Text |
|-------|------|
| BuenaRadio | "BUENA: BUENA here! Today's password! Let me think... It's [PASSWORD]! Don't forget it! I'm in GOLDENROD's RADIO TOWER!" |
| BuenaRadioMidnight | "BUENA: Oh my... It's midnight! I have to shut down! Thanks for tuning in to the end! But don't stay up too late! Presented to you by DJ BUENA! I'm outta here!" |

### Enemy Trainer Item Use
| Label | Text |
|-------|------|
| EnemyWithdrew | "[ENEMY] withdrew [MON]!" |
| EnemyUsedOn | "[ENEMY] used [ITEM] on [MON]!" |

### Repel / Item Found
| Label | Text |
|-------|------|
| RepelWoreOff | "REPEL's effect wore off." |
| PlayerFoundItem | "[PLAYER] found [ITEM]!" |
| ButNoSpace | "But [PLAYER] has no space left..." |

### Save Text
| Label | Text |
|-------|------|
| SavingRecord | "SAVING RECORD... DON'T TURN OFF!" |

---

## Common Text 2 (data/text/common_2.asm)

### NPC Trade Dialogue (3 variants)
Each NPC trade has 6 text blocks: intro, cancel, wrong Pokemon, complete, after-trade, and cable-connect.

**Variant 1:** "I collect #MON. Do you have [MON]? Want to trade it for my [MON]?"
**Variant 2:** "Hi, I'm looking for this #MON. If you have [MON], would you trade it for my [MON]?"
**Variant 3:** "[MON]'s cute, but I don't have it. Do you have [MON]? Want to trade it for my [MON]?"

### Name Rater Dialogue
| Label | Text |
|-------|------|
| NameRaterHello | "Hello, hello! I'm the NAME RATER. I rate the names of #MON. Would you like me to rate names?" |
| NameRaterBetterName | "Hm... [NAME]... That's a fairly decent name. But, how about a slightly better nickname?" |
| NameRaterPerfectName | "Hm... [NAME]? What a great name! It's perfect." |
| NameRaterFinished | "That's a better name than before! Well done!" |
| NameRaterEgg | "Whoa... That's just an EGG." |
| NameRaterSameName | "It might look the same as before, but this new name is much better!" |

### EXP and Level Text
| Label | Text |
|-------|------|
| Gained | "[MON] gained" |
| BoostedExpPoints | "a boosted [N] EXP. Points!" |
| ExpPoints | "[N] EXP. Points!" |
| **BUG:** | Five-digit experience gain is printed incorrectly |

### Pokemon Switch/Send Out Text (4 variants)
- "Go! [MON]!"
- "Do it! [MON]!"
- "Go for it, [MON]!"
- "Your foe's weak! Get'm, [MON]!"

### Pokemon Recall Text (4 variants)
- "[MON] that's enough! Come back!"
- "[MON] OK! Come back!"
- "[MON] good! Come back!"
- "[MON] come back!"

### TM/HM Text
| Label | Text |
|-------|------|
| BootedTM | "Booted up a TM." |
| BootedHM | "Booted up an HM." |
| ContainedMove | "It contained [MOVE]. Teach [MOVE] to a #MON?" |
| TMHMNotCompatible | "[MOVE] is not compatible with [MON]. It can't learn [MOVE]." |

### Day Care Text
| Label | Text |
|-------|------|
| DayCareManIntro | "I'm the DAY-CARE MAN. Want me to raise a #MON?" |
| DayCareLadyIntro | "I'm the DAY-CARE LADY. Should I raise a #MON for you?" |
| YourMonHasGrown | "Your [MON] has grown a lot. By level, it's grown by [N]. If you want your #MON back, it will cost $[amount]." |
| BackAlready | "Huh? Back already? Your [MON] needs a little more time with us. If you want your #MON back, it will cost $100." |
| FoundAnEgg | "Ah, it's you! We were raising your #MON, and my goodness, were we surprised! Your #MON had an EGG!" |

### Breeding Compatibility Messages (5 levels)
| Label | Text |
|-------|------|
| BrimmingWithEnergy | "It's brimming with energy." |
| NoInterest | "It has no interest in [MON]." |
| AppearsToCareFor | "It appears to care for [MON]." |
| Friendly | "It's friendly with [MON]." |
| ShowsInterest | "It shows interest in [MON]." |

### Mom's Savings System
| Label | Text |
|-------|------|
| MomLeavingText | "So, you're leaving on an adventure... OK! I'll help too. I know! I'll save money for you." |
| MomIsThisAboutYourMoney | "Hi! Welcome home! Or is this about your money?" |
| MomBankWhatDoYouWantToDo | "What do you want to do?" |
| MomStoreMoney | "How much do you want to save?" |
| MomTakeMoney | "How much do you want to take?" |
| MomStoredMoney | "Your money's safe here! Get going!" |
| MomTakenMoney | "[PLAYER], don't give up!" |

### Bug Catching Contest
| Label | Text |
|-------|------|
| BugCatchingContestTimeUp | "ANNOUNCER: BEEEP! Time's up!" |
| BugCatchingContestIsOver | "ANNOUNCER: The Contest is over!" |
| ContestCaughtMon | "Caught [MON]!" |
| ContestAskSwitch | "Switch #MON?" |
| ContestAlreadyCaught | "You already caught a [MON]." |
| ContestJudgingFirstPlace | "This Bug-Catching Contest winner is... [TRAINER], who caught a [MON]! The winning score was [N] points!" |

### Magikarp Guru
| Label | Text |
|-------|------|
| MagikarpGuruMeasure | "Let me measure that MAGIKARP. ...Hm, it measures [SIZE]." |
| KarpGuruRecord | "CURRENT RECORD [SIZE] caught by [TRAINER]" |

### Lucky Number Match
| Label | Text |
|-------|------|
| LuckyNumberMatchParty | "Congratulations! We have a match with the ID number of [MON] in your party." |
| LuckyNumberMatchPC | "Congratulations! We have a match with the ID number of [MON] in your PC BOX." |

### PC System Text
| Label | Text |
|-------|------|
| PokecenterPCTurnOn | "[PLAYER] turned on the PC." |
| PokecenterPCWhose | "Access whose PC?" |
| PokecenterBillsPC | "BILL's PC accessed. #MON Storage System opened." |
| PokecenterPlayersPC | "Accessed own PC. Item Storage System opened." |
| PokecenterOaksPC | "PROF.OAK's PC accessed. #DEX Rating System opened." |
| WasSentToBillsPC | "[MON] was sent to BILL's PC." |

### Mail System
| Label | Text |
|-------|------|
| EmptyMailbox | "There's no MAIL here." |
| MailClearedPutAway | "The cleared MAIL was put away." |
| MailMessageLost | "The MAIL's message will be lost. OK?" |
| MailDetached | "MAIL detached from [MON]." |
| MailSentToPC | "The MAIL was sent to your PC." |
| MailboxFull | "Your PC's MAILBOX is full." |

### Seer / Fortune Teller (Cianwood)
| Label | Text |
|-------|------|
| SeerSeeAll | "I see all. I know all... Certainly, I know of your #MON!" |
| SeerNameLocation | "Hm... I see you met [MON] here: [LOCATION]!" |
| SeerTimeLevel | "The time was [TIME]! Its level was [LEVEL]! Am I good or what?" |
| SeerTrade | "Hm... [MON] came from [OT] in a trade?" |
| SeerEgg | "Hey! That's an EGG! You can't say that you've met it yet..." |

### Seer Happiness Ratings (5 tiers)
| Label | Text |
|-------|------|
| SeerMoreCare | "It would be wise to raise your #MON with a little more care." |
| SeerMoreConfident | "It seems to have grown a little. [MON] seems to be becoming more confident." |
| SeerMuchStrength | "[MON] has grown. It's gained much strength." |
| SeerMighty | "It certainly has grown mighty! This [MON] must have come through numerous #MON battles." |
| SeerImpressed | "I'm impressed by your dedication. It's been a long time since I've seen a #MON as mighty as this [MON]." |

### Evolution Text
| Label | Text |
|-------|------|
| Evolving | "What? [MON] is evolving!" |
| EvolvedInto | "Congratulations! Your [MON] evolved into [MON]!" |
| StoppedEvolving | "Huh? [MON] stopped evolving!" |

### Move Learning Text
| Label | Text |
|-------|------|
| LearnedMove | "[MON] learned [MOVE]!" |
| AskForgetMove | "[MON] is trying to learn [MOVE]. But [MON] can't learn more than four moves. Delete an older move to make room for [MOVE]?" |
| MoveForgetCount | "1, 2 and... Poof!" |
| MoveForgot | "[MON] forgot [MOVE]. And..." |
| MoveCantForgetHM | "HM moves can't be forgotten now." |
| StopLearningMove | "Stop learning [MOVE]?" |
| DidNotLearnMove | "[MON] did not learn [MOVE]." |

### Move Deleter
| Label | Text |
|-------|------|
| DeleterIntro | "Um... Oh, yes, I'm the MOVE DELETER. I can make #MON forget moves. Shall I make a #MON forget?" |
| DeleterForgotMove | "Done! Your #MON forgot the move." |
| DeleterEgg | "An EGG doesn't know any moves!" |

### Held Item Text
| Label | Text |
|-------|------|
| PokemonSwapItem | "Took [MON]'s [ITEM] and made it hold [ITEM]." |
| PokemonHoldItem | "Made [MON] hold [ITEM]." |
| PokemonNotHolding | "[MON] isn't holding anything." |
| PokemonTookItem | "Took [ITEM] from [MON]." |
| PokemonAskSwapItem | "[MON] is already holding [ITEM]. Switch items?" |
| ItemCantHeld | "This item can't be held." |
| AnEggCantHoldAnItem | "An EGG can't hold an item." |

### Field Move HM Text
| Label | Text |
|-------|------|
| UseCut | "[MON] used CUT!" |
| CutNothing | "There's nothing to CUT here." |
| AskCut | "This tree can be CUT! Want to use CUT?" |
| BlindingFlash | "A blinding FLASH lights the area!" |
| UsedSurf | "[MON] used SURF!" |
| CantSurf | "You can't SURF here." |
| AlreadySurfing | "You're already SURFING." |
| AskSurf | "The water is calm. Want to SURF?" |
| UseWaterfall | "[MON] used WATERFALL!" |
| AskWaterfall | "Do you want to use WATERFALL?" |
| UseDig | "[MON] used DIG!" |
| UseStrength | "[MON] used STRENGTH!" |
| AskStrength | "A #MON may be able to move this. Want to use STRENGTH?" |
| BouldersMayMove | "Boulders may now be moved!" |
| UseWhirlpool | "[MON] used WHIRLPOOL!" |
| AskWhirlpool | "A whirlpool is in the way. Want to use WHIRLPOOL?" |
| UseHeadbutt | "[MON] did a HEADBUTT!" |
| AskHeadbutt | "A #MON could be in this tree. Want to HEADBUTT it?" |
| UseRockSmash | "[MON] used ROCK SMASH!" |
| AskRockSmash | "This rock looks breakable. Want to use ROCK SMASH?" |

### Fishing Text
| Label | Text |
|-------|------|
| RodBite | "Oh! A bite!" |
| RodNothing | "Not even a nibble!" |

### Item Use Text
| Label | Text |
|-------|------|
| PlayedFlute | "Played the # FLUTE. Now, that's a catchy tune!" |
| FluteWakeUp | "All sleeping #MON woke up." |
| UseSweetScent | "[MON] used SWEET SCENT!" |
| SweetScentNothing | "Looks like there's nothing here..." |
| UseSacredAsh | "[PLAYER]'s #MON were all healed!" |
| Itemfinder | "Yes! ITEMFINDER indicates there's an item nearby." |
| ItemfinderNope | "Nope! ITEMFINDER isn't responding." |
| SquirtbottleNothing | "[PLAYER] sprinkled water. But nothing happened..." |
| RepelUsedEarlierIsStillInEffect | "The REPEL used earlier is still in effect." |

### Whiteout Text
| Label | Text |
|-------|------|
| WhitedOut | "[PLAYER] is out of useable #MON! [PLAYER] whited out!" |
| PoisonFaint | "[MON] fainted!" |
| PoisonWhiteout | "[PLAYER] is out of useable #MON! [PLAYER] whited out!" |

### Bicycle Text
| Label | Text |
|-------|------|
| GotOnBike | "[PLAYER] got on the [BIKE]." |
| GotOffBike | "[PLAYER] got off the [BIKE]." |
| CantGetOffBike | "You can't get off here!" |
| NoCycling | "Cycling isn't allowed here." |

### Oak Warning Text
| Label | Text |
|-------|------|
| OakThisIsntTheTime | "OAK: [PLAYER]! This isn't the time to use that!" |
| BadgeRequired | "Sorry! A new BADGE is required." |
| CantUseItem | "Can't use that here." |

---

## Common Text 3 (data/text/common_3.asm)

### Mart/Shop Text
| Label | Text |
|-------|------|
| MartWelcome | "Welcome! How may I help you?" |
| MartHowMany | "How many?" |
| MartFinalPrice | "[N] [ITEM](S) will be $[PRICE]." |
| MartThanks | "Here you are. Thank you!" |
| MartNoMoney | "You don't have enough money." |
| MartPackFull | "You can't carry any more items." |
| MartCantBuy | "Sorry, I can't buy that from you." |
| MartComeAgain | "Please come again!" |
| MartAskMore | "Can I do anything else for you?" |
| MartBought | "Got $[PRICE] for [ITEM](S)." |
| NothingToSell | "You don't have anything to sell." |
| MartSellPrice | "I can pay you $[PRICE]. Is that OK?" |

### Herbal Medicine Shop Text
| Label | Text |
|-------|------|
| HerbShopLadyIntro | "Hello, dear. I sell inexpensive herbal medicine. They're good, but a trifle bitter. Your #MON may not like them. Hehehehe..." |
| HerbalLadyThanks | "Thank you, dear. Hehehehe..." |
| HerbalLadyComeAgain | "Come again, dear. Hehehehe..." |

### Bargain Shop Text
| Label | Text |
|-------|------|
| BargainShopIntro | "Hiya! Care to see some bargains? I sell rare items that nobody else carries--but only one of each item." |
| BargainShopSoldOut | "You bought that already. I'm all sold out of it." |

### Pharmacy Text
| Label | Text |
|-------|------|
| PharmacyIntro | "What's up? Need some medicine?" |
| PharmacyThanks | "Thanks much!" |
| PharmacyComeAgain | "All right. See you around." |

### Game Corner Text
| Label | Text |
|-------|------|
| SlotsBetHowManyCoins | "Bet how many coins?" |
| SlotsStart | "Start!" |
| SlotsLinedUp | "[SYMBOL] lined up! Won [N] coins!" |
| SlotsDarn | "Darn!" |
| SlotsPlayAgain | "Play again?" |
| SlotsNotEnoughCoins | "Not enough coins." |
| SlotsRanOutOfCoins | "Darn... Ran out of coins..." |

### Card Flip Game
| Label | Text |
|-------|------|
| CardFlipPlayWithThreeCoins | "Play with three coins?" |
| CardFlipChooseACard | "Choose a card." |
| CardFlipPlaceYourBet | "Place your bet." |
| CardFlipShuffled | "The cards have been shuffled." |
| CardFlipYeah | "Yeah!" |
| CardFlipDarn | "Darn..." |
| CardFlipPlayAgain | "Want to play again?" |

### Coin Vendor
| Label | Text |
|-------|------|
| CoinVendorIntro | "Do you need some game coins? It costs $1000 for 50 coins. Do you want some?" |
| CoinVendorBuy50 | "Thank you! Here are 50 coins." |
| CoinVendorBuy500 | "Thank you! Here are 500 coins." |
| CoinVendorCoinCaseFull | "Whoops! Your COIN CASE is full." |

### Save System Text
| Label | Text |
|-------|------|
| WouldYouLikeToSaveTheGame | "Would you like to save the game?" |
| SavingDontTurnOffThePower | "SAVING... DON'T TURN OFF THE POWER." |
| SavedTheGame | "[PLAYER] saved the game." |
| AlreadyASaveFile | "There is already a save file. Is it OK to overwrite?" |
| SaveFileCorrupted | "The save file is corrupted!" |
| ChangeBoxSave | "When you change a #MON BOX, data will be saved. OK?" |

### Battle Tower Text
| Label | Text |
|-------|------|
| BTExcuseMe | "Excuse me!" |
| ExcuseMeYoureNotReady | "Excuse me. You're not ready." |
| BattleTowerReturnWhenReady | "Please return when you're ready." |
| NeedAtLeastThreeMon | "You need at least three #MON." |
| EggDoesNotQualify | "Sorry, an EGG doesn't qualify." |
| OnlyThreeMonMayBeEntered | "Only three #MON may be entered." |
| TheMonMustAllBeDifferentKinds | "The [N] #MON must all be different kinds." |
| TheMonMustNotHoldTheSameItems | "The [N] #MON must not hold the same items." |

### Buena's Prize Text
| Label | Text |
|-------|------|
| BuenaAskWhichPrize | "Which prize would you like?" |
| BuenaIsThatRight | "[ITEM]? Is that right?" |
| BuenaHereYouGo | "Here you go!" |
| BuenaNotEnoughPoints | "You don't have enough points." |
| BuenaComeAgain | "Oh. Please come back again!" |

### Pokegear Text
| Label | Text |
|-------|------|
| GearOutOfService | "You're out of the service area." |
| PokegearAskWhoCall | "Whom do you want to call?" |
| PokegearAskDelete | "Delete this stored phone number?" |

### Phone Text
| Label | Text |
|-------|------|
| PhoneWrongNumber | "Huh? Sorry, wrong number!" |
| PhoneClick | "Click!" |
| PhoneOutOfArea | "That number is out of the area." |
| PhoneJustTalkToThem | "Just go talk to that person!" |

### Happiness Checker
| Label | Text |
|-------|------|
| HappinessText3 | "Wow! You and your #MON are really close!" |
| HappinessText2 | "#MON get more friendly if you spend time with them." |
| HappinessText1 | "You haven't tamed your #MON. If you aren't nice, it'll pout." |

### PP Up/Restore Text
| Label | Text |
|-------|------|
| RaiseThePPOfWhichMove | "Raise the PP of which move?" |
| RestoreThePPOfWhichMove | "Restore the PP of which move?" |
| PPIsMaxedOut | "[MOVE]'s PP is maxed out." |
| PPsIncreased | "[MOVE]'s PP increased." |
| PPRestored | "PP was restored." |

### Daylight Saving Time
| Label | Text |
|-------|------|
| TimesetAskDST | "Do you want to switch to Daylight Saving Time?" |
| TimesetDST | "I set the clock forward by one hour." |
| TimesetAskNotDST | "Is Daylight Saving Time over?" |
| TimesetNotDST | "I put the clock back one hour." |

### Professor Oak's Intro
| Label | Text |
|-------|------|
| OakText1 | "Hello! Sorry to keep you waiting! Welcome to the world of #MON! My name is OAK. People call me the #MON PROF." |
| OakText4 | "People and #MON live together by supporting each other. Some people play with #MON, some battle with them." |
| OakText5 | "But we don't know everything about #MON yet. There are still many mysteries to solve. That's why I study #MON every day." |
| OakText7 | "[PLAYER], are you ready? Your very own #MON story is about to unfold. You'll face fun times and tough challenges. A world of dreams and adventures with #MON awaits! Let's go!" |
| AreYouABoyOrAreYouAGirl | "Are you a boy? Or are you a girl?" |

### Pokedex Rating Messages (19 tiers)
1. "Look for #MON in grassy areas!"
2. "Good. I see you understand how to use # BALLS."
3. "You're getting good at this. But you have a long way to go."
4. "You need to fill up the #DEX. Catch different kinds of #MON!"
5. "You're trying--I can see that. Your #DEX is coming together."
6. "To evolve, some #MON grow, others use the effects of STONES."
7. "Have you gotten a fishing ROD? You can catch #MON by fishing."
8. "Excellent! You seem to like collecting things!"
9. "Some #MON only appear during certain times of the day."
10. "Your #DEX is filling up. Keep up the good work!"
11. "I'm impressed. You're evolving #MON, not just catching them."
12. "Have you met KURT? His custom BALLS should help."
13. "Wow. You've found more #MON than the last #DEX research project."
14. "Are you trading your #MON? It's tough to do this alone!"
15. "Wow! You've hit 200! Your #DEX is looking great!"
16. "You've found so many #MON! You've really helped my studies!"
17. "Magnificent! You could become a #MON professor right now!"
18. "Your #DEX is amazing! You're ready to turn professional!"
19. "Whoa! A perfect #DEX! I've dreamt about this! Congratulations!"

---

## Standard Text (data/text/std_text.asm)

### Pokemon Center Nurse (time-of-day greetings)
| Time | Text |
|------|------|
| Morning | "Good morning! Welcome to our #MON CENTER." |
| Day | "Hello! Welcome to our #MON CENTER." |
| Night | "Good evening! You're out late. Welcome to our #MON CENTER." |

### Pokemon Center Dialogue
| Label | Text |
|-------|------|
| NurseAskHeal | "We can heal your #MON to perfect health. Shall we heal your #MON?" |
| NurseTakePokemon | "OK, may I see your #MON?" |
| NurseReturnPokemon | "Thank you for waiting. Your #MON are fully healed." |
| NurseGoodbye | "We hope to see you again." |
| NursePokerus | "Your #MON appear to be infected by tiny life forms. Your #MON are healthy and seem to be fine. But we can't tell you anything more at a #MON CENTER." |

### Bookshelf Text (3 variants)
| Label | Text |
|-------|------|
| DifficultBookshelf | "It's full of difficult books." |
| PictureBookshelf | "A whole collection of #MON picture books!" |
| MagazineBookshelf | "#MON magazines... #MON PAL, #MON HANDBOOK, #MON GRAPH..." |

### Interactable Object Text
| Label | Text |
|-------|------|
| TeamRocketOath | "TEAM ROCKET OATH: Steal #MON for profit! Exploit #MON for profit! All #MON exist for the glory of TEAM ROCKET!" |
| IncenseBurner | "What is this? Oh, it's an incense burner!" |
| MerchandiseShelf | "Lots of #MON merchandise!" |
| TownMap | "It's the TOWN MAP." |
| Window | "My reflection! Lookin' good!" |
| TV | "It's a TV." |
| Homepage | "#MON JOURNAL HOME PAGE... It hasn't been updated..." |
| TrashCan | "There's nothing in here..." |
| PokecenterSign | "Heal Your #MON! #MON CENTER" |
| MartSign | "For All Your #MON Needs #MON MART" |

### Gym Statue Text
| Label | Text |
|-------|------|
| GymStatueCityGym | "[CITY] #MON GYM" |
| GymStatueWinningTrainers | "LEADER: [NAME] WINNING TRAINERS: [PLAYER]" |

### Contest Results
| Label | Text |
|-------|------|
| ReadyToJudge | "We will now judge the #MON you've caught. We have chosen the winners! Are you ready for this?" |
| PlayerWonAPrize | "[PLAYER], the No.[N] finisher, wins [PRIZE]!" |
| JoinUsNextTime | "Please join us for the next Contest!" |
| ConsolationPrize | "Everyone else gets a BERRY as a consolation prize!" |

---

## Name Input Characters (data/text/name_input_chars.asm)

### Standard Name Entry
**Lowercase:** a-z, special: x ( ) : ; [ ] PK MN
**Uppercase:** A-Z, special: - ? ! / . ,

### Box Name Entry (extended)
**Lowercase:** a-z, special: e 'd 'l 'm 'r 's 't 'v 0-9
**Uppercase:** A-Z, special: x ( ) : ; [ ] PK MN - ? ! male female / . , &

---

## Mail Input Characters (data/text/mail_input_chars.asm)

### Mail Entry
**Uppercase:** A-Z, special: , ? ! 1-9 0 PK MN PO KE e male female yen ... x
**Lowercase:** a-z, special: . - / 'd 'l 'm 'r 's 't 'v & ( ) " [ ] ' : ;

---

## Unused Text

### Sweet Honey (data/text/unused_sweet_honey.asm)
Cut feature later implemented in Diamond/Pearl. Reworked into Sweet Scent.

| Label | Text |
|-------|------|
| SweetHoney | "My #MON is an expert at collecting SWEET HONEY. I'll share some with you." |
| SweetHoneyGive | "Here you go! Have some SWEET HONEY!" |
| GotSweetHoney | "[PLAYER] received SWEET HONEY." |
| SweetHoneyAfter1 | "My little brother takes SWEET HONEY and goes somewhere with it. I wonder what he's up to?" |
| SweetHoneyAfter2 | "Did you put SWEET HONEY on a tree? What happened to it?" |
| SweetHoneyAfter3 | "Did you put SWEET HONEY on a tree? It takes about a day for #MON to be drawn to it." |
| SweetHoneyButterfree | "BUTTERFREE: Freeh!" |

### Gen 1 Trainer Names (data/text/unused_gen1_trainer_names.asm)
Untranslated Japanese trainer class names from Red version. Contains 21 class name entries in Japanese (katakana/hiragana): Youngster, Bug Catcher, Lass, Jr Trainer M/F, Pokemaniac, Super Nerd, Burglar, Engineer, Jack, Swimmer, Beauty, Rocker, Juggler, Blackbelt, Prof. Oak, Chief, Scientist, Rocket, Cooltrainer M/F.

### Dakutens (data/text/unused_dakutens.asm)
Japanese dakuten/handakuten character mapping tables (voiced consonant marks). Unused in English version. Contains hiragana and katakana mappings for consonant voicing.

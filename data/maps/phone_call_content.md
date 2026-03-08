# Pokemon Crystal - Phone Call System

Complete phone contact roster and call types from pokecrystal data/phone/.

Source: `data/phone/phone_contacts.asm`, `data/phone/special_calls.asm`, `data/phone/text/`

---

## Permanent Contacts (cannot be deleted)

| Contact | Location | Can Call You | You Call Them |
|---------|----------|-------------|--------------|
| Mom | Player's House 1F | No | Anytime |
| Prof. Elm | Elm's Lab | Yes | Anytime |

## Special Contacts

| Contact | Location | Can Call You | You Call Them |
|---------|----------|-------------|--------------|
| Bill | N/A | Yes | Anytime |
| Bike Shop | Oak's Lab | No (calls once) | No |
| Buena | Goldenrod Dept Store Roof | Yes | Anytime |

---

## Trainer Phone Contacts (30 trainers)

All trainer contacts can both call you and be called, at any time of day.

| # | Trainer | Class | Location | Route |
|---|---------|-------|----------|-------|
| 1 | Jack | Schoolboy | National Park | -- |
| 2 | Beverly | Pokefan F | National Park | -- |
| 3 | Huey | Sailor | Olivine Lighthouse 2F | -- |
| 4 | Gaven | Cooltrainer M | Route 26 | 26 |
| 5 | Beth | Cooltrainer F | Route 26 | 26 |
| 6 | Jose | Bird Keeper | Route 27 | 27 |
| 7 | Reena | Cooltrainer F | Route 27 | 27 |
| 8 | Joey | Youngster | Route 30 | 30 |
| 9 | Wade | Bug Catcher | Route 31 | 31 |
| 10 | Ralph | Fisher | Route 32 | 32 |
| 11 | Liz | Picnicker | Route 32 | 32 |
| 12 | Anthony | Hiker | Route 33 | 33 |
| 13 | Todd | Camper | Route 34 | 34 |
| 14 | Gina | Picnicker | Route 34 | 34 |
| 15 | Irwin | Juggler | Route 35 | 35 |
| 16 | Arnie | Bug Catcher | Route 35 | 35 |
| 17 | Alan | Schoolboy | Route 36 | 36 |
| 18 | Dana | Lass | Route 38 | 38 |
| 19 | Chad | Schoolboy | Route 38 | 38 |
| 20 | Derek | Pokefan M | Route 39 | 39 |
| 21 | Tully | Fisher | Route 42 | 42 |
| 22 | Brent | Pokemaniac | Route 43 | 43 |
| 23 | Tiffany | Picnicker | Route 43 | 43 |
| 24 | Vance | Bird Keeper | Route 44 | 44 |
| 25 | Wilton | Fisher | Route 44 | 44 |
| 26 | Kenji | Blackbelt | Route 45 | 45 |
| 27 | Parry | Hiker | Route 45 | 45 |
| 28 | Erin | Picnicker | Route 46 | 46 |

---

## Phone Call Types

Each trainer has 3 script files: `_callee.asm` (you call them), `_caller.asm` (they call you), and `_overworld.asm` (in-person dialogue for registration).

### Call Categories

Trainer calls fall into these categories:

#### 1. Rematch Requests
Trainers call to challenge you to a rematch at their location.
- Example (Joey): "Let's get together and battle! I promise things will be different! [Route 30] is where I'll be. Give me a shout when you come."
- Reminder: "What's keeping you, <PLAYER>! Let's get down and battle already!"

#### 2. Pokemon Sightings / Wild Encounters
Trainers tell you about Pokemon they've seen or defeated.
- Example (Joey): "Oh yeah, I took down a [Pokemon] in the wild the other day. It was a cakewalk."
- Or: "Oh yeah, I saw a wild [Pokemon]! I thought about going for it, but I decided to work with my one-and-only."

#### 3. Training Updates
Trainers tell you about training their Pokemon.
- Example (Joey): "My [Pokemon]'s looking sharper than before! I doubt there's a Pokemon as cool as this guy in your party!"
- Example (Joey): "I'm checking out [Pokemon]'s moves and devising some strategies."

#### 4. Item Gifts
Certain trainers call to offer you items they've found.
- Beverly: Nugget
- Jose: Star Piece
- Wade: Various items (Berry, PSNCureBerry, etc.)
- Gina: Leaf Stone
- Alan: Fire Stone
- Liz: Thunderstone
- Derek: Nugget
- Tully: Water Stone
- Tiffany: Pink Bow
- Wilton: Various items

#### 5. Swarm Reports
Some trainers report rare Pokemon swarms:
- Ralph (Route 32): Qwilfish swarm
- Arnie (Route 35): Yanma swarm
- Chad (Route 38): Snubbull info
- Wilton (Route 44): Remoraid info

#### 6. General Chat / Tips
Many calls are flavor text about the trainer's life, battle philosophy, or game tips. Irwin is infamous for calling frequently with useless chatter.

---

## Special Phone Calls (Story Triggers)

From `data/phone/special_calls.asm`. These are scripted calls triggered at specific story points:

| # | Caller | Condition | Script |
|---|--------|-----------|--------|
| 1-4 | Prof. Elm | Only when outside | Various story updates |
| 5 | Prof. Elm | Wherever you are | Urgent story call |
| 6 | Bike Shop | Wherever you are | Bike is ready |
| 7 | Mom | Wherever you are | Lecture about saving money |
| 8 | Prof. Elm | Only when outside | Story update |

---

## Key NPC Phone Scripts

### Mom
- Manages your savings account (deposit/withdraw)
- Reports items she bought with your money
- Asks about Daylight Savings Time transitions

### Prof. Elm
- Calls about discoveries, egg hatching progress
- Story-critical calls (stolen Pokemon, egg discovery)
- Calls when you've caught Pokerus

### Bill
- Calls when your PC box is full
- "Your PC BOX is full! You should switch BOXES."

### Buena
- Radio personality, calls about her Password show
- Gives quiz questions for Blue Card points

---

## Phone Call Timing

All registered trainer contacts can call at `ANYTIME` (morning, day, or night). The game checks a daily flag per trainer for:
- **Rematch readiness** (24 trainers have rematch flags)
- **Item availability** (10 trainers have item gift flags)
- **Time-of-day flags** (24 trainers tracked)

Calls are randomly triggered when transitioning between routes. The game limits concurrent calls to prevent spam.

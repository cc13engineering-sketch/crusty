# MYCELIA

## Pitch

You are a fungal network spreading through a procedurally generated cave, absorbing nutrients, splitting into branching hyphae connected by spring joints, and racing to colonize the cavern before a creeping blight consumes it. A biological strategy game where every tendril you grow is a physics-simulated chain that sways, snaps, and pulses with flowing resources.

## Features Showcased (10 systems composing together)

| # | Engine Feature | How It's Used |
|---|---------------|---------------|
| 1 | **Procedural Generation** (cellular automata) | Cave terrain generated each run from a random seed -- organic walls, nutrient pockets, blight origin |
| 2 | **TileMap + Rendering** | Cave rendered as colored tile grid; nutrient tiles glow warm, blight tiles pulse dark red |
| 3 | **A* Pathfinding** | Blight AI uses pathfinding to find shortest route toward your network's closest node |
| 4 | **Physics Joints** (spring/rope) | Each hypha segment is a chain of entities connected by spring joints -- the network sways and stretches physically |
| 5 | **Force Fields** (attract/repel) | Nutrient hotspots emit attraction fields that gently pull nearby hypha tips; blight zones repel |
| 6 | **Resource Inventories** | Every node has a ResourceInventory with "energy" and "nutrients" slots; production/consumption rates drive growth decisions |
| 7 | **Visual Connections** (FlowLine) | Animated flow-line connections between nodes show resources pulsing through the network -- color shifts from green (healthy) to yellow (starving) |
| 8 | **Graph Nodes** | Entity relationship graph tracks the network topology: which nodes feed which, strongest supply edges, colony groups |
| 9 | **Signal System** | Nutrient deposits emit signals on channels; receiver nodes detect when a deposit is nearby and trigger growth toward it |
| 10 | **Environment Clock** | A "decay cycle" with phases (Fertile / Dormant / Blight Surge) that change resource production rates and blight speed over time |
| 11 | **State Machines** | Each hypha tip has states: Growing / Absorbing / Dormant / Dying, with transitions driven by resource levels and blight proximity |
| 12 | **Property Tweens** | Node scale pulses with BounceOut easing when absorbing nutrients; hyphae tips oscillate with ping-pong scale tweens while growing |
| 13 | **Particles** | Spore bursts when a node absorbs a nutrient pocket; dark particle clouds where blight consumes terrain |
| 14 | **Ghost Trails** | Growing hypha tips leave fading green afterimages showing recent growth direction |
| 15 | **Post-FX + Screen FX** | Vignette darkens cave edges; screen flash on blight surge; desaturate tint when network is starving; screen shake when a branch snaps |
| 16 | **Per-Entity Time Scale** | Nodes near blight slow down (time dilation representing cellular stress); nodes near nutrient hotspots speed up |
| 17 | **Hierarchy** (parent-child) | Each hypha branch is a parent-child chain; severing a parent node kills all downstream children |

## Gameplay Loop

1. **Seed Phase**: Cave generates. Your spore lands in a random room. You see the cave, nutrient deposits glowing amber, and a distant red blight origin.

2. **Growth Phase** (core loop):
   - Click/tap on an empty cave tile adjacent to your network to grow a new hypha segment toward it
   - A* finds the tile-path; spring-joint chain physically extends along it
   - Each node costs energy from its parent; energy flows visually through FlowLine connections
   - Nutrient deposits get absorbed on contact, refilling the local node's ResourceInventory
   - You choose: grow wide (more territory, thinner supply lines) or grow deep (fortify fewer paths)

3. **Blight Phase** (escalating threat):
   - Blight spreads from its origin each "Blight Surge" phase of the environment clock
   - Blight uses A* pathfinding toward your nearest node
   - Blight converts cave tiles to dead tiles (dark red), destroying any nutrients there
   - If blight reaches a node, the node enters "Dying" state -- its spring joints weaken and can snap (break_force threshold), severing downstream branches
   - Severed branches flash (EntityFlash), desaturate (ScreenFX), and fade out

4. **Decision Tension**:
   - Grow toward nutrients before blight eats them
   - Sacrifice branches as blight shields to protect the core
   - Manage energy flow: overextended networks starve and go Dormant

## Visual Design

- **Color Palette**: Deep cave blacks/dark blues for walls. Warm amber/gold for nutrients. Vivid green for healthy mycelium. Sickly yellow for starving nodes. Deep crimson/dark red for blight. White spore particle bursts.
- **The Network**: Spring-joint chains physically sway and bounce. FlowLine dots animate along connections showing resource direction. Node scale pulses rhythmically (breathing effect via ping-pong tween). Growing tips leave ghost trail afterimages.
- **The Cave**: Cellular automata walls with organic irregular shapes. Nutrient pockets are clusters of warm-colored tiles. Blight creep visually overtakes tiles frame-by-frame.
- **Atmosphere**: Vignette creates underground claustrophobia. Environment clock shifts ambient tint -- Fertile phases brighten slightly, Blight Surge phases darken and add red tint. Screen shake when joints snap. Particle clouds at points of conflict.
- **Camera**: Smooth follow on the player's cursor / most recently grown tip, with zoom that pulls out as the network expands.

## Win / Lose Conditions

**WIN**: Colonize 75% of the cave's nutrient deposits before the blight reaches your root node. Victory triggers a green-gold screen flash, all nodes pulse with ElasticOut scale tween, spore particle burst from every node.

**LOSE**: Blight reaches and destroys your root node (the original spore). The network chain-collapses via parent-child hierarchy -- branches snap one by one with screen shake, desaturation sweeps across the screen, particles scatter from each dying node.

**Score**: Total nutrients absorbed + percentage of cave colonized + time survived. Displayed as bitmap text HUD.

## Why This Demo

This game is impossible to build with any single engine feature. The fun emerges from the *composition*:
- Without physics joints, the network would be static lines -- the spring sway gives it life
- Without resource inventories + flow lines, growth would be free -- the supply chain creates strategy
- Without A* + tilemap, the blight would be random -- directed threat creates tension
- Without environment clock, the pacing would be flat -- cyclical surges create rhythm
- Without graph nodes, severing a branch couldn't cascade -- topology-aware destruction creates drama
- Without signals, growth would be blind -- directional hints create navigation

Every feature listed above is load-bearing. Remove any one and the game meaningfully degrades. That is the definition of composability.

Hire agents, think carefully, execute with precision. Ask questions if necessary. Operate with autonomy of 8. Only work on the music theory game. Here are the improvements/changes/fixes I would like you to research and create a plan for - 

- Let's do ANKI style, making use of spaced repetition logic to determine which challenge to show. Keep this principle in mind when proposing UI changes:
  - The Mental Model for "Progress". Anki doesn't give you an XP bar or completion %. The real progress signal is: Your daily due count staying manageable + mature card % rising over weeks. Consistency beats cramming — the system rewards daily reviews over heroic sessions.
  - NOTE: this means we'll likely need to support persistence at the game engine level - start with a way to do CRUD operations against the browser's local storage - keep the persistence interface general enough to later work in different persistence "backends"

- We need some every so slight fade in/out - like very tiny - on the sounds that play in the challenge sequence - right now there's a little popping, staggered feeling to the audio - use your best judgement here.

- We also want some kind of "hint" concept - consider deeply the current layout to make that look nice. Maybe the hint is something that exposes a modal or a expanded view - I'm not sure what is best in our webgl setup.

- The scroll bar below the keyboard is just a tiny bit too close to the keys. I have often mistakenly scrolled when I meant to hit the keys - on the topic of the scrollbar - I should be able to drag it all the way to the end on either side - right now its a bit hard - maybe some space to the right and left of the scrollbar would solve this. After moving the scroll bar down slightly - make it taller and easier to use too.

- Currently the "composed phrase" section doesn't make a lot of sense. Ditch it entirely and replace with some clean visual to "replay challenge sound" this is the better space for it then the [ replay ] text button above.

- Remove references to musictheory.net, or any other external link for that matter.

- the part at the bottom footer - "Difficulty", "Phrase" and "Mode" - aren't adding value. Just ditch them - maybe we can link to something to help the user understand what the ANKI model for learning is - ultimately that's they key thing a new user might need to know.

- Minor bug - on mobile make sure tapping, even if briefly, on the challenge answer option, always selects the option. Due to how we treat tapping like hovering - mobile is just a bit unintuitive (make sense?)

- Remove the "score" concept entirely - instead add any stats, if any are even needed maybe not, that give some visual indication of the "ANKI" progress or work or whatever. 
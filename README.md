# The Caverns of the Shadow Queen

A turn-based, procedurally-generated roguelike.

Built using Rust, `bracket-lib`, `legion`, and _Hands-on Rust_ by Herbert Wolverson as a part of [r/roguelikedev](https://old.reddit.com/r/roguelikedev)'s annual event.

### Table of Contents

- [RoguelikeDev Does The Complete Roguelike Tutorial](#roguelikedev-does-the-complete-roguelike-tutorial)
- Weekly Implementation Notes
	- [Week 1](#week-1) ✅
	- [Week 2](#week-2) ✅
	- [Week 3](#week-3)
	- [Week 4](#week-4) ✅
	- [Week 5](#week-5)
	- [Week 6](#week-6)
	- [Week 7](#week-7)
	- [Week 8](#week-8)
- [Design Specification](#design-specification)
	- [Story](#story)
	- [Game Loop](#game-loop)
	- [MVP Goals](#mvp-goals)
	- [Stretch Goals](#stretch-goals)

## RoguelikeDev Does The Complete Roguelike Tutorial

![RoguelikeDev 2024 Logo](./assets/roguelikedev-2024.png)

This repository is participating in the 2024 edition of
the [RoguelikeDev event](https://old.reddit.com/r/roguelikedev/comments/1dt8bqm/roguelikedev_does_the_complete_roguelike_tutorial/),
running from 9 July 2024 till 27 August 2024. The event is structured to follow the [canonical Python tutorial](https://rogueliketutorials.com/tutorials/tcod/v2/). _Hands-on Rust_ only loosely follows the structure of the tutorial, so the steps may be a little out of order.

### [Week 1](https://old.reddit.com/r/roguelikedev/comments/1dz7bbg/roguelikedev_does_the_complete_roguelike_tutorial/)

✅ [Progress comment](https://old.reddit.com/r/roguelikedev/comments/1dz7bbg/roguelikedev_does_the_complete_roguelike_tutorial/lcvcfbm/) | [Milestone commit](https://github.com/avinashv/rl-cotsq/commit/87ebf7c68887df61bb9c8ef689c4b51ced5b0350)

<details>
<summary>9 July 2024 - Setup, drawing an @, and moving around</summary>

 - I have some experience with the [original tutorial](http://bfnightly.bracketproductions.com/rustbook/) that Herbert Wolverson made before this book. I find that so far, the book is easier for me to grasp as someone still relatively inexperienced with Rust.
 - The fact that I know that ECS is incoming means I already know that a major refactor is incoming which is frustration that I am going to face--at this point in the book I'd rather have just dealt with `legion` being implemented up-front.

</details>

### [Week 2](https://old.reddit.com/r/roguelikedev/comments/1e4qhsb/roguelikedev_does_the_complete_roguelike_tutorial/)

✅ [Progress comment](https://old.reddit.com/r/roguelikedev/comments/1e4qhsb/roguelikedev_does_the_complete_roguelike_tutorial/ldqor7i/) | [Milestone commit](https://github.com/avinashv/rl-cotsq/commit/d689405efaa4a4e0e579372c5e9d77839804e4e4)

<details>
<summary>16 July 2024 - The object system and generating your first map</summary>

 - Started work on this a bit early.
 - Map implementation was the boring rooms-and-corridors method that I really dislike, because it results in very unnatural structure generation. Reading ahead, I can see there are some more interesting approaches such as cellular automata later on, so I am not jumping ahead for now.
 - The implementation of a camera is quite straightforward.
 - Not following the original Python tutorial here at all, instead the book uses `legion` to provide ECS.
   - Injecting resources into the `World` is clever, and the way that the camera and map become a resource that a query result can interact with is very nice.

</details>

### Week 3

✅ [Progress comment]() | [Milestone commit](https://github.com/avinashv/rl-cotsq/commit/dea1fcddfaa28f97d5a3d468df2c1d62723c31d0)

<details>
<summary>23 July 2024 - Field of view, placing enemies, and attacking</summary>

 - FOV is implemented very late in this book, so I will leave that till the end.
 - The ECS system shines in this simple situation--enemies are just defined and placed and all the tying together of rendering is already done.
 - Again, I just get the feeling there is a lot of refactoring that is going to be done later and typing all of this redundant code feels like busy-work when I know there is going to be some instruction like, "delete it".
 - I like the way state is managed, and a different system scheduler for each state is very interesting.
   - Once it is refactored into an intent-based system, **there is a bug**. `build_monster_scheduler()` doesn't call `collisions::collisions_system()` after the `movement::movement_system()` is called (as it is in the player's scheduler) and if you wait the monsters will eventually randomly find themselves on the player and in that move itself they should be destroyed. Currently they just sit on the player until a movement from the player is processed--and that movement has to be "wait".
   - This gets refactored out entirely as this entire system gets removed.

</details>

### Week 4

✅ Progress comment (TBD) | [Milestone commit](https://github.com/avinashv/rl-cotsq/commit/dea1fcddfaa28f97d5a3d468df2c1d62723c31d0)

<details>
<summary>30 July 2024 - Combat damage and building the interface</summary>

 - Doing everything here in [Week 3](#week-3).
 - The UI implementation in the book doesn't work for me. I've currently laid it out a bit more like a traditional roguelike with the pane along the bottom.
   - I also don't like the tooltip on mouse hover. I've currently got that showing in the pane as well.
 - Another week, another refactor. This time the deletion of `collisions.rs`. I am happy to see the intents system that was implemented with `WantsToMove` be reimplemented with `WantsToAttack`. This consistency is great.

</details>

### Week 5

<details>
<summary>6 August 2024 - Items, inventory and ranged targeting</summary>

TBC

</details>

### Week 6

<details>
<summary>13 August 2024 - Save/load and leveling up</summary>

TBC

</details>

### Week 7

<details>
<summary>20 August 2024 - Monster/item progression and equipment</summary>

TBC

</details>

### Week 8

<details>
<summary>27 August 2024 - Sharing your game</summary>

TBC

</details>

## Design Specification

### Story

TBD.

### Game Loop

<details>

- Enter dungeon level
- Explore, revealing the map
- Encounter enemies; fight or flee
- Find items to gain benefits
- Locate the exit to the level

</details>

### MVP Goals

<details>

- [x] Player can walk around
- [x] Create a basic procedural dungeon map
- [ ] Player has field-of-view
- [x] Spawn monsters
- [x] Players can fight monsters
- [ ] Add items and inventory
- [ ] Add a win condition
- [ ] Game over when the player dies

</details>

### Stretch Goals

<details>

- [x] Camera system
- [ ] Bitset walls
- [ ] Monsters have field-of-view
- [ ] Messaging log
- [ ] Entities have a basic finite state machine
- [ ] Add more interesting dungeon designs
- [ ] Add dungeon themes
- [ ] Add multiple dungeon levels
- [ ] Add weapons
- [ ] Data-driven monster design
- [ ] Visual effects for combat
- [ ] Scoring system
- [ ] Sneaking system to see around corners
- [ ] More complex monster AI

</details>
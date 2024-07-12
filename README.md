# The Caverns of the Shadow Queen

A turn-based, procedurally-generated roguelike.

Built using Rust, `bracket-lib`, `legion`, and _Hands-on Rust_ by Herbert Wolverson.

### Table of Contents

- [RoguelikeDev Does The Complete Roguelike Tutorial](#roguelikedev-does-the-complete-roguelike-tutorial)
- Weekly Implementation Notes
	- [Week 1](#week-1)
	- [Week 2](#week-2)
	- [Week 3](#week-3)
	- [Week 4](#week-4)
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
the [RoguelikeDev event](https://www.reddit.com/r/roguelikedev/comments/1dt8bqm/roguelikedev_does_the_complete_roguelike_tutorial/),
running from 9 July 2024 till 27 August 2024.

### [Week 1](https://www.reddit.com/r/roguelikedev/comments/1dz7bbg/roguelikedev_does_the_complete_roguelike_tutorial/)

<details>
<summary>9 July 2024 - Setup, drawing an @, and moving around</summary>

 - I have some experience with the [original tutorial](http://bfnightly.bracketproductions.com/rustbook/) that Herbert Wolverson made before this book. I find that so far, the book is easier for me to grasp as someone still relatively inexperienced with Rust.
 - The fact that I know that ECS is incoming means I already know that a major refactor is incoming which is frustration that I am going to face--at this point in the book I'd rather have just dealt with `legion` being implemented up-front.

</details>

### Week 2

<details>
<summary>16 July 2024 - The object system and generating your first map</summary>

TBC

</details>

### Week 3

<details>
<summary>23 July 2024 - Field of view, placing enemies, and attacking</summary>

TBC

</details>

### Week 4

<details>
<summary>30 July 2024 - Combat damage and building the interface</summary>

TBC

</details>

### Week 5

<details>
<summary>6 August 2024 - Items, inventory and ranged targeting</summary>

TBC

</details>

### Week 6

<details>
<summary>13 August 2024 - <del>Save/load and leveling up</del> Item usage</summary>

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

- [ ] Player can walk around
- [ ] Create a basic procedural dungeon map
- [ ] Player has field-of-view
- [ ] Spawn monsters
- [ ] Players can fight monsters
- [ ] Add items and inventory
- [ ] Add a win condition
- [ ] Game over when the player dies

</details>

### Stretch Goals

<details>

- [ ] Camera system
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
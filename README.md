# Rust-Dungeon-Crawler
This repo is inteded to hold the source code and resources for the dungeoncrawler game made in Rust with the bracket-lib crate and the Legion ECS system.
<br />
The game was developed through the book *Hands-on Rust*, by Herbert Wolverson (ISBN 978-1-68050-816-1). Through this book/exercise, I learned of several important aspects of game design, such as Entity Component Systems and data-driven design, as well as procedural generation and code re-use. Following the book, the bulk of the work on this project was a series of significant expansions on the original work, with the *Hands-on Rust* work serving as foundational code. All artwork and audio are attributed within this file, and below is a condensed list of significant additions made to the original code.
<br />
<br />
## Ideas for Expansion
* [x] Change tileset from fantasy to sci-fi/horror theme <br />
* [x] Increase enemy variety and complexity <br />
* [x] Introduce new map elements (locked doors, keys, etc.) <br />
### Introduce new items for gameplay: <br />
* [x] Item to increase player's Field of Vision attribute <br />
* [x] Attack armor: shield player from incoming damage until armor is depleted <br />
* [x] Introduce environmental hazards: <br />
* [x] Poison floor / fire floor (could require special item to traverse safely) <br />
### Other Mechanics: <br />
* [x] Add a scoring mechanism <br />
* [ ] Add gameplay music and/or sound fx
<br />

## Stretch Goals 
If time permits, I would like to look into these more advanced concepts: <br />
* [ ] Shift into real-time gameplay instead of turn-based <br />
* [ ] Look into Bevy game engine for possible improvements to performance <br />
* [ ] Add more detail to the game's story with an intro/outro screen <br />
<br />

## Crates used:
bracket-lib = ~0.8.1
<br />
legion = 0.3.1
<br />
serde = 1.0.115
<br />
ron = 0.6.1
rodio = 0.15.0

## Attributions:
General idea and foundational code: *Hands-on Rust* by Herbert Wolverson. Copyright 2021. The Pragmatic Programmers, LLC. Print. (ISBN: 978-1-68050-816-1)
<br />
#### Artwork:
https://opengameart.org/content/unfinished-dungeon-tileset
<br />
https://opengameart.org/content/fantasy-magic-set
<br />
https://opengameart.org/content/fantasy-sword-set
<br />
http://opengameart.org/content/dungeon-crawl-32x32-tiles
<br />

#### Audio:
https://opengameart.org/content/16-monster-growls
<br />
https://opengameart.org/content/monster-sound-effects-pack
<br />
https://opengameart.org/content/horror-sound-effects-library 
(Little Robot Sound Factory: www.littlerobotsoundfactory.com)
<br />
https://opengameart.org/content/rpg-sound-pack


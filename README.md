# Rust-Dungeon-Crawler
Repo for dungeoncrawler game built in preparation for CSCI 7200
<br />
This repo is inteded to hold the source code and resources for the dungeoncrawler game made in Rust with the bracket-lib crate and the Legion ECS system. It is primarily for demonstrational purposes to show where my work in CSCI 7200 began, as I plan on expanding on this game over the Summer of 2022.
<br />
The game was developed through the book "Hands-on Rust", by Herbert Wolverson (ISBN 978-1-68050-816-1). Through this book/exercise, I learned of several important aspects of game design, such as Entity Component Systems and data-driven design, as well as procedural generation.
<br />
<br />
## Ideas for Expansion
* [ ] Change tileset from fantasy to sci-fi/horror theme <br />
* [ ] Increase enemy variety and complexity <br />
* [ ] Introduce new map elements (locked doors, keys, etc.) <br />
* [ ] Itroduce new items for gameplay: <br />
*&emsp;&emsp;[ ] NV Goggles: increase player FOV and apply green tint to environment <br />
*&emsp;&emsp;[x] Attack armor: shield player from incoming damage until armor is depleted <br />
* [ ] Introduce environmental hazards: <br />
*&emsp;&emsp;[ ] Poison gas - could require gas mask item to traverse <br />
* [ ] Add a scoring mechanism <br />
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


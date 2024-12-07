# Tomb Of The Mask

A Tomb Of The Mask clone written in Bevy and Rust. Another one for the 20 games challenge, which I think should turn into the [A Game A Week](https://www.gamedeveloper.com/audio/game-a-week-getting-experienced-at-failure#A%20Game%20A%20Week) challenge, because that's more the spirit we're trying to capture here.

## Goal

* ~~Create at least one roguelike dungeon leve~~
* ~~Create a player character that can only move up, right, down and left on the respective keypresses. A direction keypress will make the character move in that direction until they reach the next obstacle (i.e. a wall)~~
* ~~Add an exit tile to the dungeon level, to complete the leve~~
* Add spikey walls that will kill the player if they stand in front of them for too long (i.e. more than half a second)


## Stretch goal

* Fill the dungeon with PacMan style coins that the player collects on passing them 
* Add screen shake 
* Add more levels
* Other stuff from the original Tomb Of The Mask
* Release to iOS, so we need to move the character on swipe gestures as well

## Or a minor pivot to make it fresh

* Game/Movement mechanics from TotM
* To complete a level, the player has to be in a certain configuration. Configurations are: Start, Square, Circle, Pentagon etc. 
* Passing certain tiles on the playing field changes the configuration 
* Level can only be completed when in a required configuration
* Some game element is needed to increase the difficulty as time passes, so there is an urgency to complete a level

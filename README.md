# Bashing Bytes

## Introduction

This is a simple Roguelike, that is about as unbalanced and simple as it gets. This project served as a way for me to learn how to use the bracket library, as well as how to structure a long term project.

I am satisified with how the project is at this point, even though it could use some cleaning up, and half the settings page doesn't work.

The project taught me a lot about the different libraries in Rust, like Serde, BracketLib, and others, as they can be found in my Toml file. Additionally, it led to my first published library EnumCycle, as I got tired of writing Boilerplate code for the different menus.

## Gameplay

Bashing Bytes has 3 enemies, each with different stats configurable in the spawns.ron file. Items can be added in the same file, although new abilities cannot be added without going into the code. Items that can currently be found:
 - Fireball Scroll
 - Magic Missile Scroll
 - Simple Dagger
 - Simple Shield (which makes you invincible to all but orcs)
 - Health Potions

You can move around and explore all the maps that will be generated. If you happen to find a '<<', while standing on it, you can press '.' to go deeper in the dungeon.

## Future of Bashing Bytes

I don't honestly know if I will be coming back to it. It has served its purpose, and it was a fun project to take me through a large portion of the pandemic. I may make changes every once in a while, as ideas pop into my head. But it is, as of the time of this writing, unlikely that I return to make large changes, and edit it further.

## Conclusion

Thanks for taking the time to read this. If you find errors, or things that I should change in the code, please let me know and I will get back to you as quick as I can.

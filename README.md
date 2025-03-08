# sea2d
Project authors: Bor Brudar, Lara Velkavrh

## Running instructions 

The program can be run as either the server, the client, or both at the same time by using `cargo r server`, `cargo r client` or just 
`cargo r` respectively. The client command takes an optional argument, namely the IP address and the port it should try to connect to, 
separated by a colon, e.g. `cargo r client 127.0.0.1:6000`. By running on localhost the default port is 6000. Remember to have devel versions
of sdl2 and sdl2_image installed on your system or you won't be able to compile the program. Arch users: `sudo pacman -S sdl2-compat sdl2_image`,
everyone else figure it out :P

## General description:

A 2D multiplayer (2-player) computer game:
- level based game with puzzle solving,
- pixel art aesthetics.



### Storyline:
You are one of the scientists on a research trip to the depths of the ocean when suddenly your submarine stops operating. 

It is your and your partner's job to navigate your way out of the malfunctioned vessel, escape dangers of the vast ocean and find a way back to civilization after being standed on a tropical island. 

And perhaps even solve some ancient mysteries in the process...

![Current state of the game](resources/screenshots/image.png)
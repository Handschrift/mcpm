# About the project
Mcpm is a CLI package manager for minecraft which currently only supports searching for mods.
This is an early version so features and / or code improvements might be added later.
This project was initially started by me because I wanted to learn the rust programming language. If you find any improvements regarding the code-quality please note them with an issue or a pull request.
# Installing
Clone the project and run the following command: ``cargo build``.

Make sure that you have a recent version of the rust compiler installed!
# Usage
Currently, mcpm only supports searching and downloading mods at [Modrinth](https://modrinth.com/).

You first have to init a minecraft instance. To do that execute the following command inside your .minecraft directory:
``mcpm init``
This will prompt you to answer some questions like the minecraft version and the modloader you would like to use.

Then you can search mods with ``mcpm search <name of the mod>``

This will output a list of mods matching the name you searched for including a mod slug you need to download a mod. You can download it with:
``mcpm install <mod slug>``
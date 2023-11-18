# dub_dump
Dub Dump is a tool for quickly reviewing and discarding audio clips

# Usage
Launch the app, it will ask for a directory.

Once you provide it a directory full of audio files, it will clone the directory one step up (ie `c:\Voiceover Audio` will be cloned into `c:\Dub Dump`).

Right now, the directory cloning does not check to see if there is already a copy of the files, and the app will crash if there is, so you need to delete the old `Dub Dump` folder before running again on the same directory.

Once all the audio has been cloned, the app will take a moment to index the audio, then it will drop you into the main screen.

On the main screen, controls are shown at the bottom.

More info about the screen layout can be found in [title_screen.rs](src/terminal_functions/title_screen.rs) at the top of the file.

`d: dump` will immediately delete the file with no way to undo it, so be careful! (I do plan to add an undo feature eventually though)
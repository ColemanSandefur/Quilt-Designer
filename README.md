# Quilt-Designer
A simple program that will help you design a quilt

## How do I use it?
When you launch the application you will see images on the left bar and block patterns on the right bar.
When you select a block pattern, whichever block you then click on the quilt will become that block pattern.
Likewise, when you select a color or image, wherever you click will be filled with that color or image.
To use custom images, put the custom images in the images folder and relaunch the application.

You can also save and load quilts through the menubar at the top. Saves are usually located in the saves folder
You can safely move saves to different computers and load them.

# Developer Section

## Building instructions
To build run `cargo run`
the binary will be located at target/debug/quilt_builder

## Technologies
This was built using glium and lyon for graphics
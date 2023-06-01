# world-painter

This is a rewrite of my previous
[WorldPainter](https://github.com/funnyboy-roks/WorldPainter) originally
written in Java, now written in Rust.

This tool takes in any number of directories and outputs them as an
image showing the chunks, coloured by their size.

![Image of output](./img/example.png)

The above image was formed using the following command:

```sh
$ world-painter \
~/server/world/region 25000 '#005207' \
~/server/world_nether/DIM-1/region 20000 '#370101' \
~/server/world_the_end/DIM1/region 25000 '#0f0022'
```

## Usage

```sh
$ world-painter [<path> <world-border> <color>]...
```

One can specify however many worlds they wish to read, but all must have
the path, worldborder, and colour in that order.

## Todo

- [ ] Better user interface
    - [ ] Better logging
    - [ ] Perhaps a file which can be read for the config rather than
      just having the command-line
    - [ ] Figure out how one can use clap to make the cli _much_ nicer
      (I'd be surprised if what I wanted to do was not possible)
- [ ] Zip & Upload image to [bytebin](https://bytebin.lucko.dev)
    - If bytebin says no because the image is too large, just write
      it to the disk.

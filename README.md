# AOCFUSE

A [FUSE](https://en.wikipedia.org/wiki/Filesystem_in_Userspace) implementation
to turn the puzzle inputs from [AoC 2022 Day 7](https://adventofcode.com/2022/day/7)
into an actual filesystem you can browse like you would any other.

It implements only the basic functionality of a filesystem, that which is required
by the [AoC 2022 Day 7](https://adventofcode.com/2022/day/7) puzzle. More specifically it allows you
to move through the directories
as defined in the input, as well as seeing their contents and observing file sizes.

> ***Do note that this project is simply for fun and has no practical use. The implementation
is not exactly robust and I take no responsibility for any damage that may occur.***

## Usage

**Dependencies** 
 - The [Rust programming language](https://www.rust-lang.org/)
 - [fuse] and [libfuse] as discussed on the [`fuser` GitHub repository](https://github.com/cberner/fuser#dependencies)

The filesystem can be mounted by running `cargo r -- <aoc_input_file> <mount_location>` in the
project's root directory. The `<aoc_input_file>` must be a text file containing any puzzle input
following the [AoC 2022 Day 7](https://adventofcode.com/2022/day/7) input format.
An example can be found on the respective website.

## License

Licensed under the [BSD 3-Clause License](./LICENSE).

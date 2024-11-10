# Conway's Game of life in Rust

A simple implementation of this game in Rust. At 2048^2 resolution it runs at about 20-25 fps on an i5-8600K.

# Building

Use cargo build --release to build. Building a debug version won't work as it doesn't handle underflows. Also it is painfully slow in debug.

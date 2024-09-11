# Rectangle collision simulation

Using [raylib-rs](https://github.com/raylib-rs/raylib-rs) v5.0.2 as rendering engine.

For the simulation [regular grid](https://en.wikipedia.org/wiki/Regular_grid) a is used to reduce the number of checks.

If you simply launch the simulation it will run with 1200 blocks.

## TODO
- [ ] Add concurrency.
- [ ] Optimize grid insertion to don't use AABB for walls.
- [ ] Don't build the grid on each frame :)

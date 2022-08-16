# shadowcasting

2d small shadowcasting project written in Rust with
[leo10torres](https://github.com/leo10torres).

The mouse is the source of light, walls can be added by left-clicking
on the screen and removed by right clicking, when light emitted by the
mouse collisions with it's "nearest" wall, it will cease, making the
effect of a shadow.

## Implementation

Windowing and rendering is done using the [sdl2](https://docs.rs/sdl2/latest/sdl2/)
crate due to it's simplicity and lightweight-ness.

As for the shadowcasting effect, the method is fundamentally simmilar
to the one in this [javidx9's](https://youtu.be/fc3nnG2CG8U) video:

- Turn the tilemap into geometry, effectively "merging" contiguous tiles

- Cast a ray to each end of every edge and calculate it's collision point

- Render each trio (mouse) (ray1) (ray2) as a void or filled triangle

However the tilemap is just a byte-vector.

## TODO

- Turn tilemap into geometry (`to_wallmap()`)

- Apply the ray-casting

- Add dynamic rendering toggles (sdl2) and static setup configs (config.toml)

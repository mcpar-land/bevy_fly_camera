[![Crates.io](https://img.shields.io/crates/v/bevy_fly_camera)](https://crates.io/crates/bevy_fly_camera)

# bevy-fly-camera

This is a really basic flying camera bundle and plugin for Bevy. It's useful for testing games before you've coded your own movement system.

It features both a 3d camera with Minecraft-style motion, and a 2d camera without mouse-looking.

Keybinds can be edited, but the defaults are:

# 3D

- <kbd>W</kbd> / <kbd>A</kbd> / <kbd>S</kbd> / <kbd>D</kbd> - Move along the horizontal plane
- <kbd>Space</kbd> - Move upward
- <kbd>L Shift</kbd> - Move downward

```rust
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn setup(commands: &mut Commands) {
  commands
    .spawn(Camera3dBundle::default())
    .with(FlyCamera::default());
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_plugin(FlyCameraPlugin)
    .run();
}
```

[Runnable 3D Example](examples/basic.rs)

# 2D

- <kbd>W</kbd> / <kbd>A</kbd> / <kbd>S</kbd> / <kbd>D</kbd> - Move along the 2d plane

```rust
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};

fn setup(commands: &mut Commands) {
  commands
    .spawn(Camera2dBundle::default())
    .with(FlyCamera2d::default());
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_plugin(FlyCameraPlugin)
    .run();
}
```

[Runnable 2D Example](examples/2d.rs)

---

If you like this crate, there are some [issues](https://github.com/mcpar-land/bevy_fly_camera/issues/7) that I would love to get some help on to make it more maintainable!

If you use this crate in a project, I'd love to know about it! Send me a message or just open an issue about it! ♥

Any PRs are also welcome, though keep in mind that the project scope is intentionally tiny: A quick and dirty 3D motion camera, almost entirely intended for intermediate development steps or 3D demos.

---

# Version Matching

| Bevy Version | `bevy_fly_camera` Version |
| ------------ | ------------------------- |
| `0.1.0`      | `0.1.1`                   |
| `0.1.3`      | `0.3.0`                   |
| `0.2`        | `0.4.0`                   |
| `0.2.1`      | `0.4.1`                   |
| `0.3.0`      | `0.5.0`                   |
| `0.4.0`      | `0.6.0`                   |
| `0.5.0`      | `0.7.0`                   |
| `0.6.0`      | `0.8.0`                   |
| `0.9.0`      | `0.9.0`                   |
| `0.10.0`     | `0.10.0`                  |

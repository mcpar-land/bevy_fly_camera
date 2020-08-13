This is a really basic flying camera bundle and plugin for Bevy. It's useful for testing 3d games before you've coded your own movement system.

Keybinds are:

- W / A / S / D - Move along the horizontal plane
- Shift - Move downward
- Space - Move upward

# Example

```rust
use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn setup(mut commands: Commands) {
	commands.spawn(FlyCamera::default());
}

fn main() {
	App::build()
		.add_default_plugins()
		.add_startup_system(setup.system())
		.add_plugin(FlyCameraPlugin)
		.run();
}
```

Check out the [simple example](examples/basic.rs)

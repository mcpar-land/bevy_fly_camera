//! A simple plugin and bundle for a basic flying camera in Bevy.
//! Keybinds are identical to Minecraft:
//! - W / A / S / D - Move along the horizontal plane
//! - Shift - Move downward
//! - Space - Move upward
//!
//! # Example
//! ```rust
//! use bevy::prelude::*;
//! use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
//!
//! fn setup(mut commands: Commands) {
//!		commands.spawn(FlyCamera::default());
//! }
//! fn main() {
//!		App::build()
//! 		.add_default_plugins()
//! 		.add_startup_system(setup.system())
//! 		.add_plugin(FlyCameraPlugin)
//! 		.run();
//! }
//! ```
//!
//! There's also a basic piece of example code included in `/examples/basic.rs`
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_render::camera::{Camera, PerspectiveProjection, VisibleEntities};
use bevy_render::render_graph::base;

/// A set of options for initializing a FlyCamera.
pub struct FlyCameraOptions {
	/// The speed the FlyCamera moves at.
	pub speed: f32,
	/// The sensitivity of the FlyCamera's motion based on mouse movement.
	pub sensitivity: f32,
	/// The current pitch of the FlyCamera. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub pitch: f32,
	/// The current pitch of the FlyCamera. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub yaw: f32,
}
impl Default for FlyCameraOptions {
	fn default() -> Self {
		Self {
			speed: 10.0,
			sensitivity: 3.0,
			pitch: 0.0,
			yaw: 0.0,
		}
	}
}

/**
The FlyCamera bundle.
Spawn this to instantiate a flying camera.

```
fn setup(mut commands: Commands) {
	commands.spawn(FlyCamera::default());
}
```

This is based off [`Camera3dComponents`](https://docs.rs/bevy/0.1.2/bevy/prelude/struct.Camera3dComponents.html) in the base package, with added options and systems.
**/
#[derive(Bundle)]
pub struct FlyCamera {
	pub options: FlyCameraOptions,
	pub camera: Camera,
	pub perspective_projection: PerspectiveProjection,
	pub visible_entities: VisibleEntities,
	pub transform: Transform,
	pub translation: Translation,
	pub rotation: Rotation,
	pub scale: Scale,
}

impl Default for FlyCamera {
	fn default() -> Self {
		Self {
			options: FlyCameraOptions::default(),
			camera: Camera {
				name: Some(base::camera::CAMERA3D.to_string()),
				..Default::default()
			},
			perspective_projection: Default::default(),
			visible_entities: Default::default(),
			transform: Default::default(),
			translation: Default::default(),
			rotation: Default::default(),
			scale: Default::default(),
		}
	}
}

fn forward_vector(rotation: &Rotation) -> Vec3 {
	rotation.mul_vec3(Vec3::unit_z()).normalize()
}

fn forward_walk_vector(rotation: &Rotation) -> Vec3 {
	let f = forward_vector(rotation);
	let f_flattened = Vec3::new(f.x(), 0.0, f.z()).normalize();
	f_flattened
}

fn strafe_vector(rotation: &Rotation) -> Vec3 {
	// Rotate it 90 degrees to get the strafe direction
	Rotation::from_rotation_y(90.0f32.to_radians())
		.mul_vec3(forward_walk_vector(rotation))
		.normalize()
}

fn movement_axis(
	input: &Res<Input<KeyCode>>,
	plus: KeyCode,
	minus: KeyCode,
) -> f32 {
	let mut axis = 0.0;
	if input.pressed(plus) {
		axis += 1.0;
	}
	if input.pressed(minus) {
		axis -= 1.0;
	}
	axis
}

fn camera_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&FlyCameraOptions, &mut Translation, &Rotation)>,
) {
	let axis_h = movement_axis(&keyboard_input, KeyCode::D, KeyCode::A);
	let axis_v = movement_axis(&keyboard_input, KeyCode::S, KeyCode::W);

	let axis_float =
		movement_axis(&keyboard_input, KeyCode::Space, KeyCode::LShift);

	for (options, mut translation, rotation) in &mut query.iter() {
		let delta_f = forward_walk_vector(rotation)
			* axis_v
			* options.speed
			* time.delta_seconds;

		let delta_strafe =
			strafe_vector(rotation) * axis_h * options.speed * time.delta_seconds;

		let delta_float =
			Vec3::unit_y() * axis_float * options.speed * time.delta_seconds;

		translation.0 += delta_f + delta_strafe + delta_float;
		// println!("{:#?}", camera.projection_matrix);
	}
}

#[derive(Default)]
struct State {
	mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn mouse_motion_system(
	time: Res<Time>,
	mut state: ResMut<State>,
	mouse_motion_events: Res<Events<MouseMotion>>,
	mut query: Query<(&mut FlyCameraOptions, &mut Rotation)>,
) {
	let mut delta: Vec2 = Vec2::zero();
	for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
		delta += event.delta;
	}
	if delta == Vec2::zero() {
		return;
	}

	for (mut options, mut rotation) in &mut query.iter() {
		options.yaw -= delta.x() * options.sensitivity * time.delta_seconds;
		options.pitch += delta.y() * options.sensitivity * time.delta_seconds;

		if options.pitch > 89.9 {
			options.pitch = 89.9;
		}
		if options.pitch < -89.9 {
			options.pitch = -89.9;
		}
		// println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

		let yaw_radians = options.yaw.to_radians();
		let pitch_radians = options.pitch.to_radians();

		rotation.0 = Quat::from_axis_angle(Vec3::unit_y(), yaw_radians)
			* Quat::from_axis_angle(-Vec3::unit_x(), pitch_radians);
	}
}

/**
Include this plugin to add the systems for the FlyCamera bundle.

```
fn main() {
	App::build().add_plugin(FlyCameraPlugin);
}
```

**/

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.init_resource::<State>()
			.add_system(camera_movement_system.system())
			.add_system(mouse_motion_system.system());
	}
}

//! A simple plugin and components for 2d/3d flying cameras in Bevy.
//!
//! # 3D
//!
//! Movement system is based on Minecraft, flying along the horizontal plane no matter the mouse's vertical angle, with two extra buttons for moving vertically.
//!
//! Default keybinds are:
//! - <kbd>W</kbd> / <kbd>A</kbd> / <kbd>S</kbd> / <kbd>D</kbd> - Move along the horizontal plane
//! - Shift - Move downward
//! - Space - Move upward
//!
//! ## Example
//! ```no_compile
//! use bevy::prelude::*;
//! use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
//!
//! fn setup(commands: &mut Commands) {
//!	  commands
//!     .spawn(Camera3dBundle::default())
//!     .with(FlyCamera::default());
//! }
//!
//! fn main() {
//!	  App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_startup_system(setup.system())
//!     .add_plugin(FlyCameraPlugin)
//!     .run();
//! }
//! ```
//!
//! There's also a basic piece of example code included in `/examples/basic.rs`
//!
//! # 2D
//! Movement system only uses the keyboard to move in all four directions across the 2d plane.
//!
//! The default keybinds are:
//! - <kbd>W</kbd> / <kbd>A</kbd> / <kbd>S</kbd> / <kbd>D</kbd> - Move along the 2d plane
//!
//! ## Example
//! ```no_compile
//! use bevy::prelude::*;
//! use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};
//! ```
//! ```no_compile
//!	commands
//!   .spawn(Camera2dBundle::default())
//!   .with(FlyCamera2d::default());
//! ```
//!
//! There's also a basic piece of example code included in `/examples/2d.rs`

use bevy::{input::mouse::MouseMotion, prelude::*};
use cam2d::camera_2d_movement_system;
use util::movement_axis;

mod cam2d;
mod util;

pub use cam2d::FlyCamera2d;

/// A set of options for initializing a FlyCamera.
/// Attach this component to a [`Camera3dBundle`](https://docs.rs/bevy/0.4.0/bevy/prelude/struct.Camera3dBundle.html) bundle to control it with your mouse and keyboard.
/// # Example
/// ```no_compile
/// fn setup(mut commands: Commands) {
///	  commands
///     .spawn(Camera3dBundle::default())
///     .with(FlyCamera::default());
/// }

pub struct FlyCamera {
	/// The speed the FlyCamera accelerates at. Defaults to `1.0`
	pub accel: f32,
	/// The maximum speed the FlyCamera can move at. Defaults to `0.5`
	pub max_speed: f32,
	/// The sensitivity of the FlyCamera's motion based on mouse movement. Defaults to `3.0`
	pub sensitivity: f32,
	/// The amount of deceleration to apply to the camera's motion. Defaults to `1.0`
	pub friction: f32,
	/// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub pitch: f32,
	/// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub yaw: f32,
	/// The current velocity of the FlyCamera. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub velocity: Vec3,
	/// Key used to move forward. Defaults to <kbd>W</kbd>
	pub key_forward: KeyCode,
	/// Key used to move backward. Defaults to <kbd>S</kbd>
	pub key_backward: KeyCode,
	/// Key used to move left. Defaults to <kbd>A</kbd>
	pub key_left: KeyCode,
	/// Key used to move right. Defaults to <kbd>D</kbd>
	pub key_right: KeyCode,
	/// Key used to move up. Defaults to <kbd>Space</kbd>
	pub key_up: KeyCode,
	/// Key used to move forward. Defaults to <kbd>LShift</kbd>
	pub key_down: KeyCode,
	/// If `false`, disable keyboard control of the camera. Defaults to `true`
	pub enabled: bool,
}
impl Default for FlyCamera {
	fn default() -> Self {
		Self {
			accel: 1.5,
			max_speed: 0.5,
			sensitivity: 3.0,
			friction: 1.0,
			pitch: 0.0,
			yaw: 0.0,
			velocity: Vec3::ZERO,
			key_forward: KeyCode::W,
			key_backward: KeyCode::S,
			key_left: KeyCode::A,
			key_right: KeyCode::D,
			key_up: KeyCode::Space,
			key_down: KeyCode::LShift,
			enabled: true,
		}
	}
}


pub mod camera_events {
    use bevy::{math::Vec2};

	#[derive(Debug)]
	pub enum EventType {
		// Move forward or back, bool is whether to move horizontally
		// FIXME: should non-horizontal movement be a separate enum? (MoveLook?)
		Move(f32, bool),
		Strafe(f32),
		MoveVertical(f32),
		LookMouse(Vec2),
	}

	#[derive(Debug)]
	pub struct CameraEvent {
		pub event_type: EventType,
	}
}


fn forward_vector(rotation: &Quat) -> Vec3 {
	rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
	let f = forward_vector(rotation);
	let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
	f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
	// Rotate it 90 degrees to get the strafe direction
	Quat::from_rotation_y(90.0f32.to_radians())
		.mul_vec3(forward_walk_vector(rotation))
		.normalize()
}

fn emit_camera_rotation_events(
	mut mouse_motion_event_reader: EventReader<MouseMotion>,
	mut emit_events: bevy::prelude::EventWriter<camera_events::CameraEvent>,

) {
	let mut delta: Vec2 = Vec2::ZERO;
	for event in mouse_motion_event_reader.iter() {
		delta += event.delta;
	}
	if delta.is_nan() || delta == Vec2::ZERO {
		return;
	}

	emit_events.send(camera_events::CameraEvent {
		event_type: camera_events::EventType::LookMouse(delta)
	})
}

fn consume_camera_rotation_events(
	time: Res<Time>,
	mut events: EventReader<camera_events::CameraEvent>,
	mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
	for (mut options, mut transform) in query.iter_mut() {
		if !options.enabled {
			continue;
		}
		let mut delta = Vec2::ZERO;
		for event in events.iter() {
			match event.event_type {
			    camera_events::EventType::LookMouse(mouse_delta) => { delta += mouse_delta }
				_ => {}
			}
		}

		if delta != Vec2::ZERO {
		
			options.yaw -= delta.x * options.sensitivity * time.delta_seconds();
			options.pitch += delta.y * options.sensitivity * time.delta_seconds();

			options.pitch = options.pitch.clamp(-89.0, 89.9);
			// println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

			let yaw_radians = options.yaw.to_radians();
			let pitch_radians = options.pitch.to_radians();

			transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
				* Quat::from_axis_angle(-Vec3::X, pitch_radians);
		}
	}
}

fn emit_camera_movement_events(
	keyboard_input: Res<Input<KeyCode>>,
	mut emit_events: bevy::prelude::EventWriter<camera_events::CameraEvent>,
	mut query: Query<&mut FlyCamera>,
) {
	for options in query.iter_mut() {
		let (axis_h, axis_v, axis_float) = if options.enabled {
			(
				movement_axis(&keyboard_input, options.key_right, options.key_left),
				movement_axis(
					&keyboard_input,
					options.key_backward,
					options.key_forward,
				),
				movement_axis(&keyboard_input, options.key_up, options.key_down),
			)
		} else {
			(0.0, 0.0, 0.0)
		};

		if axis_h != 0.0 {
			emit_events.send(camera_events::CameraEvent {
				event_type: camera_events::EventType::Strafe(axis_h)
			})
		}

		if axis_v != 0.0 {
			emit_events.send(camera_events::CameraEvent {
				event_type: camera_events::EventType::Move(axis_v, true)
			})
		}

		if axis_float != 0.0 {
			emit_events.send(camera_events::CameraEvent {
				event_type: camera_events::EventType::MoveVertical(axis_float)
			})
		}
	}
}

fn consume_camera_events(
	time: Res<Time>,
	mut events: EventReader<camera_events::CameraEvent>,
	mut query: Query<(&mut FlyCamera, &mut Transform)>,
) {
	for (mut options, mut transform) in query.iter_mut() {
		let mut accel = Vec3::ZERO;
		let rotation = transform.rotation;

		for event in events.iter() {
			match event.event_type {
				camera_events::EventType::Move(distance, _horizontal) => {
					accel += forward_walk_vector(&rotation) * distance;
				}
			    camera_events::EventType::Strafe(distance) => {
					accel += strafe_vector(&rotation) * distance;
				}
			    camera_events::EventType::MoveVertical(distance) => {
					accel += Vec3::Y * distance
				}
			    camera_events::EventType::LookMouse(_) => {}
			}
		}

		let accel: Vec3 = if accel.length() != 0.0 {
			accel.normalize() * options.accel
		} else {
			Vec3::ZERO
		};

		let friction: Vec3 = if options.velocity.length() != 0.0 {
			options.velocity.normalize() * -1.0 * options.friction
		} else {
			Vec3::ZERO
		};

		options.velocity += accel * time.delta_seconds();

		// clamp within max speed
		if options.velocity.length() > options.max_speed {
			options.velocity = options.velocity.normalize() * options.max_speed;
		}

		let delta_friction = friction * time.delta_seconds();

		options.velocity = if (options.velocity + delta_friction).signum()
			!= options.velocity.signum()
		{
			Vec3::ZERO
		} else {
			options.velocity + delta_friction
		};

		transform.translation += options.velocity;
	}
}

/**
Include this plugin to add the systems for the FlyCamera bundle.

```no_compile
fn main() {
	App::build().add_plugin(FlyCameraPlugin);
}
```

**/

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			// .add_system(camera_movement_system.system())
			.add_system(camera_2d_movement_system.system())
			.add_system(emit_camera_rotation_events.system()
				.label("emit_camera_rotation_events"))
			.add_system(consume_camera_rotation_events.system()
				.label("consume_camera_rotation_events")
				.after("emit_camera_rotation_events"))
			.add_system(emit_camera_movement_events.system()
				.label("emit_camera_movement_events"))
			.add_system(consume_camera_events.system()
				.label("consume_camera_movement_events")
				.after("emit_camera_movement_events")
		);

		app.add_event::<camera_events::CameraEvent>();
	}
}

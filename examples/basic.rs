use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

// This is a simple example of a camera that flies around.
// There's an included example of a system that toggles the "enabled"
// property of the fly camera with "T"

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
			DirectionalLight::default(),
			Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
	));
	commands
		.spawn((
			Camera3d::default(),
			Msaa::Sample4,
		))
		.insert(FlyCamera::default());

	let box_mesh = meshes.add(Mesh::from(Cuboid::new(0.25, 0.25, 0.25)));
	let box_material = materials.add(Color::srgb(1.0, 0.2, 0.3));

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn((
					Mesh3d(box_mesh.clone()),
					MeshMaterial3d(box_material.clone()),
					Transform::from_translation(Vec3::new(
						x as f32, y as f32, z as f32,
					)),
				));
			}
		}
	}

	println!("Started example!");
}

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(
	input: Res<ButtonInput<KeyCode>>,
	mut query: Query<&mut FlyCamera>,
) {
	for mut options in query.iter_mut() {
		if input.just_pressed(KeyCode::KeyT) {
			println!("Toggled FlyCamera enabled!");
			options.enabled = !options.enabled;
		}
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, init)
		.add_plugins(FlyCameraPlugin)
		.add_systems(Update, toggle_button_system)
		.run();
}

use bevy::prelude::*;
use bevy_fly_camera::{
	FlyCamera,
	FlyCameraPlugin,
};

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn(LightComponents {
			translation: Translation::new(4.0, 8.0, 4.0),
			..Default::default()
		})
		.spawn(Camera3dComponents::default())
		.with(FlyCamera::default());

	let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
	let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn(PbrComponents {
					mesh: box_mesh,
					material: box_material,
					translation: Translation::new(x as f32, y as f32, z as f32),
					..Default::default()
				});
			}
		}
	}

	println!("Started example!");
}

fn main() {
	App::build()
		.add_resource(Msaa { samples: 4 })
		.add_default_plugins()
		.add_startup_system(init.system())
		.add_plugin(FlyCameraPlugin)
		.run();
}

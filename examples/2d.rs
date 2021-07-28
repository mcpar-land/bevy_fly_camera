use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};

fn init(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands
		.spawn()
		.insert_bundle(OrthographicCameraBundle::new_2d())
		.insert(FlyCamera2d::default());

	const AMOUNT: i32 = 6;
	const SPACING: f32 = 300.0;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			commands.spawn().insert_bundle(SpriteBundle {
				material: materials.add(asset_server.load("icon.png").into()),
				transform: Transform::from_xyz(
					x as f32 * SPACING,
					y as f32 * SPACING,
					0.0,
				),
				..Default::default()
			});
		}
	}
}

fn main() {
	App::new()
		.insert_resource(Msaa { samples: 4 })
		.add_plugins(DefaultPlugins)
		.add_startup_system(init.system())
		.add_plugin(FlyCameraPlugin)
		.run();
}

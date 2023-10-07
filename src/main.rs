//use bevy::prelude;
use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, Camera3dBundle, Color, Commands, default, Mesh, PbrBundle, ResMut, shape, StandardMaterial, Startup, Transform, Update, Vec3, Visibility};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::prelude::ShapePlugin;
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::DiscPainter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ShapePlugin::default())

        .add_systems(Startup, init_demo)
        .add_systems(Update, draw_shape_test)
        .run();
}

fn init_demo(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    //commands.spawn(Camera2dBundle::default());
    //commands.spawn(Camera3dBundle::default());
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -20.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    // red color
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        }),
        visibility: Visibility::Visible,
        ..default()
    });
}

fn draw_shape_test(mut painter: ShapePainter) {
    // Draw a circle
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.circle(5.0);

}
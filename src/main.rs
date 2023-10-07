//use bevy::prelude;
use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, Camera3dBundle, Color, Commands, default, Mesh, PbrBundle, Res, ResMut, Resource, shape, StandardMaterial, Startup, Transform, Update, Vec3, Visibility};
use bevy_inspector_egui::egui::Painter;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::prelude::{LinePainter, ShapePlugin};
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::DiscPainter;

#[derive(Clone, Copy, Default)]
pub enum XOXGrid{
    #[default]
    Empty,
    X,
    O,
}

#[derive(Resource, Default)]
pub struct XOXBoard{
    pub grid: [XOXGrid; 9],
    pub score: u32,
}




fn main() {
    let mut xox_board = XOXBoard{
        grid : [XOXGrid::Empty; 9],
        score : 1,
    };
    xox_board.grid[0] = XOXGrid::X;
    xox_board.grid[1] = XOXGrid::O;
    xox_board.grid[2] = XOXGrid::O;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ShapePlugin::default())

        .insert_resource(xox_board)

        .add_systems(Startup, init_demo)
        .add_systems(Update,draw_xox_board)
        .run();
}

fn init_demo(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -20.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    //// spawn red color cube
    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //    material: materials.add(StandardMaterial {
    //        base_color: Color::rgb(1.0, 0.0, 0.0),
    //        ..default()
    //    }),
    //    visibility: Visibility::Visible,
    //    ..default()
    //});
}
fn draw_xox_board(board: Res<XOXBoard>, mut painter: ShapePainter) {
    draw_grid(&mut painter);

    // iterate
    board.grid.iter().enumerate().for_each(|(index, grid)| {
        let x = index % 3;
        let y = index / 3;
        let position = Vec3::new(x as f32 * 5.0 - 5.0, y as f32 * 5.0 - 5.0, 0.0);
        match grid {
            XOXGrid::Empty => {}
            XOXGrid::X => draw_x(position, &mut painter),
            XOXGrid::O => draw_o(position, &mut painter),
        }
    });
}
fn draw_o(position : Vec3, painter: &mut ShapePainter){
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.transform = Transform::from_translation(position);
    painter.circle(2.25);
}

fn draw_x(position : Vec3, painter: &mut ShapePainter){
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.transform = Transform::from_translation(position);

    let line_len = 1.75;
    painter.line(Vec3::new(-line_len, line_len, 0.0), Vec3::new(line_len, -line_len, 0.0));
    painter.line(Vec3::new(line_len, line_len, 0.0), Vec3::new(-line_len, -line_len, 0.0));
}
fn draw_grid(painter: &mut ShapePainter){
    painter.transform = Transform::from_translation(Vec3::ZERO);
    painter.thickness = 0.5;
    painter.color = Color::rgb(1.0, 1.0, 1.0);
    painter.line(Vec3::new(-2.5, 5.0, 0.0), Vec3::new(-2.5, -5.0, 0.0));
    painter.line(Vec3::new(2.5, 5.0, 0.0), Vec3::new(2.5, -5.0, 0.0));
    painter.line(Vec3::new(-5.0, 2.5, 0.0), Vec3::new(5.0, 2.5, 0.0));
    painter.line(Vec3::new(-5.0, -2.5, 0.0), Vec3::new(5.0, -2.5, 0.0));
}


/*
fn draw_shape_test(mut painter: ShapePainter) {
    // Draw a circle
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.circle(5.0);
}

 */
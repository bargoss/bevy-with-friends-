mod utils;
use utils::*;

use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, Camera, Camera3dBundle, Color, Commands, default, EventReader, GlobalTransform, info, Mesh, MouseButton, PbrBundle, Res, ResMut, Resource, shape, StandardMaterial, Startup, Transform, Update, Vec3, Visibility, Window};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::prelude::{LinePainter, ShapePlugin};
use bevy_vector_shapes::prelude::ShapePainter;
use bevy_vector_shapes::shapes::DiscPainter;
use bevy::ecs::system::Query;
use bevy::input::ButtonState::Pressed;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::IntoSystemConfigs;

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
#[derive(Resource, Default)]
pub struct UserInput{
    pub clicked: Option<Vec3>,
}


const GRID_LEN: f32 = 5.0;

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
        .insert_resource(UserInput::default())

        .add_systems(Startup, init_demo)
        .add_systems(Update,draw_xox_board)
        .add_systems(Update, take_user_input)
        .run();
}

fn take_user_input(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut user_input: ResMut<UserInput>,
    mut click_events: EventReader<MouseButtonInput>,
) {
    let window = windows.single();
    if let Some(screen_pos) = window.cursor_position() {
        let cam_entity = camera_query.single();
        let camera = cam_entity.0;
        let transform = cam_entity.1;

        let plane = Plane{
            normal: Vec3::Z,
            point: Vec3::ZERO,
        };

        user_input.clicked = None;

        if let Some(ray) = camera.viewport_to_world(transform, screen_pos) {
            if let Some(hit) = ray_plane_intersection(&ray, &plane) {
                if click_events.iter().any(|event| event.button == MouseButton::Left && event.state == Pressed) {
                    let position = hit;
                    user_input.clicked = Some(position);
                    info!("Mouse Position: {:?}", position);
                    let (x, y) = world_to_grid_xy(position);
                    let index = grid_xy_to_grid_index(x, y);
                    info!("Mouse Position: {:?}", index);
                }
            }
        }
    }
}

fn init_demo(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
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
        let position = Vec3::new(x as f32 * GRID_LEN - GRID_LEN, y as f32 * GRID_LEN - GRID_LEN, 0.0);
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
    painter.circle(0.45 * GRID_LEN);
}

fn draw_x(position : Vec3, painter: &mut ShapePainter){
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.transform = Transform::from_translation(position);

    let line_len = 0.35 * GRID_LEN;
    painter.line(Vec3::new(-line_len, line_len, 0.0), Vec3::new(line_len, -line_len, 0.0));
    painter.line(Vec3::new(line_len, line_len, 0.0), Vec3::new(-line_len, -line_len, 0.0));
}
fn draw_grid(painter: &mut ShapePainter){
    painter.transform = Transform::from_translation(Vec3::ZERO);
    painter.thickness = 0.5;
    painter.color = Color::rgb(1.0, 1.0, 1.0);
    painter.line(Vec3::new(-GRID_LEN*0.5, GRID_LEN, 0.0), Vec3::new(-GRID_LEN*0.5, -GRID_LEN, 0.0));
    painter.line(Vec3::new(GRID_LEN*0.5, GRID_LEN, 0.0), Vec3::new(GRID_LEN*0.5, -GRID_LEN, 0.0));
    painter.line(Vec3::new(-GRID_LEN, GRID_LEN*0.5, 0.0), Vec3::new(GRID_LEN, GRID_LEN*0.5, 0.0));
    painter.line(Vec3::new(-GRID_LEN, -GRID_LEN*0.5, 0.0), Vec3::new(GRID_LEN, -GRID_LEN*0.5, 0.0));
}


/*
fn draw_shape_test(mut painter: ShapePainter) {
    // Draw a circle
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.circle(GRID_LEN);
}

 */

fn world_to_grid_xy(position: Vec3) -> (i32,i32){
    let x = position.x;
    let y = position.y;

    let pivot = Vec3::new(-GRID_LEN * 1.5, -GRID_LEN*1.5, 0.0);
    let delta_from_pivot = position - pivot;

    let x_index = (delta_from_pivot.x / GRID_LEN) as i32;
    let y_index = (delta_from_pivot.y / GRID_LEN) as i32;

    (x_index, y_index)
}
fn grid_xy_to_grid_index(x: i32, y: i32) -> usize{
    (y * 3 + x) as usize
}
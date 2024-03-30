pub mod geom_funcs;
pub use crate::geom_funcs::*;
pub mod fps_capper;
pub use crate::fps_capper::*;
pub mod geo_objects;
pub use crate::geo_objects::*;
pub mod util_funcs;
pub use crate::util_funcs::*;
use crossterm::{cursor, execute};
use std::f32::consts::PI;
use std::f32::EPSILON;
use std::io::stdout;
pub mod vec_methods;


const SCREEN_SIZE: usize = 64;
const FPS: u8 = 60;
const SCREEN_DISTANCE: f32 = 25.0;
const OBJ_DISTANCE: f32 = 30.0;
const TEXTURE: &'static str = ".-=;+!*SX#$@";
const LIGHT_SRC: [f32; 3] = [100.0, 100.0, 100.0];


fn main()
{
    let mut capper = FpsCapper::new(FPS);
    let mut frame_buffer: Vec<Vec<char>> = vec![vec![' '; SCREEN_SIZE]; SCREEN_SIZE];
    let mut z_buffer: Vec<f32> = vec![0.0 as f32; SCREEN_SIZE * SCREEN_SIZE]; // uses 1/z

    let mut a: f32 = 0.0; // manage change in rotation
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;

    let torus: Torus = Torus::new(16.0, 4.0, 3.0, 3.0, 3.0);
    let torus_points = Torus::get_points_to_draw(&torus, PI / 100.0, PI / 100.0);
    let hedron: Tetrahedron = Tetrahedron::new(
        vec![0.0    , 10.0  , 0.0],
        vec![0.0    , -10.0 , 10.0],
        vec![-10.0  , -10.0 , -10.0],
        vec![10.0   , -10.0 , -10.0]
    );
    let hedron_points = Tetrahedron::get_points_to_draw(&hedron, 100);
    let shpere: Sphere = Sphere::new(10.0, -5.0, 5.0, 5.0);
    let sphere_points = Sphere::get_points_to_draw(&shpere, PI / 100.0, PI / 100.0);
    let all_points: Vec<Vec<Vec<f32>>> = vec![torus_points, hedron_points, sphere_points];

    let mut figure_change_counter: usize = 0;
    let mut points_of_curr_obj = all_points[figure_change_counter].clone();

    // main render loop
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout(),cursor::Hide).unwrap();
    let mut running = true;
    while running
    {
        FpsCapper::start_measurement(&mut capper);
        
        if e_pressed() // change 3D model to show
        {
            figure_change_counter = (figure_change_counter + 1) % all_points.len();
            points_of_curr_obj = all_points[figure_change_counter].clone();
        }

        let points_to_draw: Vec<Vec<f32>> = calc_rotation(&points_of_curr_obj, a, b, c);

        for ptd in points_to_draw
        {
            if ptd[2] + OBJ_DISTANCE < EPSILON { continue; }

            let z_inv: f32 = 1.0 / (ptd[2] + OBJ_DISTANCE);
            let x_projected: f32 = (SCREEN_SIZE / 2) as f32 + SCREEN_DISTANCE * ptd[0] * z_inv;
            let y_projected: f32 = (SCREEN_SIZE / 2) as f32 - SCREEN_DISTANCE * ptd[1] * z_inv;
            let x_idx: usize = f32::round(x_projected) as usize;
            let y_idx: usize = f32::round(y_projected) as usize;

            if x_idx >= SCREEN_SIZE || y_idx >= SCREEN_SIZE { continue; }

            let buffer_index: usize = x_idx + y_idx * SCREEN_SIZE;
            if z_inv <= z_buffer[buffer_index] { continue; }

            z_buffer[buffer_index] = z_inv;

            let point_texture = TEXTURE.chars().nth(11).unwrap();
            // TODO: surface normal texture index ^^^^
            frame_buffer[y_idx][x_idx] = point_texture;
        }

        print_screen(&frame_buffer);
        clear_frame_buffer(&mut frame_buffer);
        clear_z_buffer(&mut z_buffer);
        a += 0.02;
        b += 0.02;
        c += 0.02;

        FpsCapper::cap_fps(&mut capper);
        if space_pressed() { running = false; }
    }

    execute!(stdout(), cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    pause();
}

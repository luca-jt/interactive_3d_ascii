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
use std::io::stdout;
pub mod vec_methods;


const SCREEN_W: usize = 64;
const SCREEN_H: usize = 32;
const FPS: u8 = 60;
const RENDER_SPACE_SIZE: f32 = 50.0; // 100x100x100 | (50,50,50) is max distance
const SCREEN_DISTANCE: f32 = 10.0; // change that to your liking
const TEXTURE: &'static str = ".-:,=;<+!c*z?svilu2SwkP694OGAXH8R#$B0M%@";
const LIGHT_SRC: [f32; 3] = [RENDER_SPACE_SIZE, RENDER_SPACE_SIZE, RENDER_SPACE_SIZE];


fn main()
{
    let mut capper = FpsCapper::new(FPS);
    let mut frame_buffer: Vec<Vec<char>> = vec![vec![' '; SCREEN_W]; SCREEN_H];
    let mut z_buffer: Vec<Vec<f32>> = vec![vec![0.0 as f32; SCREEN_W]; SCREEN_H]; // use 1/z !
    let mut a: f32 = 0.0; // manage change in rotation
    let mut b: f32 = 0.0; // "
    let mut c: f32 = 0.0; // "

    let torus: Torus = Torus::new(10.0, 3.0, 0.0, 0.0, 0.0);
    let torus_points = Torus::get_points_to_draw(&torus, PI / 10.0, PI / 20.0);
    let hedron: Tetrahedron = Tetrahedron::new(vec![], vec![], vec![], vec![]); // TODO: punkte eintragen
    let hedron_points = Tetrahedron::get_points_to_draw(&hedron, 50);
    let shpere: Sphere = Sphere::new(10.0, 0.0, 0.0, 0.0);
    let sphere_points = Sphere::get_points_to_draw(&shpere, PI / 20.0, PI / 20.0);
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

        let points_to_draw: Vec<Vec<f32>> = calc_rotation(&points_of_curr_obj, a, b, c)
                                            .into_iter()
                                            .filter(|p| 
                                                   p[0] <= RENDER_SPACE_SIZE
                                                && p[0] >= -RENDER_SPACE_SIZE
                                                && p[1] <= RENDER_SPACE_SIZE
                                                && p[1] >= -RENDER_SPACE_SIZE
                                                && p[2] <= RENDER_SPACE_SIZE
                                                && p[2] >= -RENDER_SPACE_SIZE)
                                            .collect();

        for ptd in points_to_draw
        {
            let mapped_point: Vec<f32> = vec![
                f32::round(SCREEN_DISTANCE * ptd[0] / ptd[2]),
                f32::round(SCREEN_DISTANCE * ptd[1] / ptd[2])
                ];
                
            // map the points to the screen grid using buffers + textures + surface normal
        }

        print_screen(&frame_buffer);
        clear_frame_buffer(&mut frame_buffer);
        clear_z_buffer(&mut z_buffer);
        a += 0.005;
        b += 0.005;
        c += 0.005;

        FpsCapper::cap_fps(&mut capper);
        if space_pressed() { running = false; }
    }

    execute!(stdout(), cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    pause();
}

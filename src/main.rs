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


const DISTANCE_FROM_SCREEN: u32 = 10; // change that to your liking
const FPS: u8 = 60;
const TEXTURE: &str = ".-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const SCREEN_W: usize = 64;
const SCREEN_H: usize = 24;
const RENDER_SPACE_SIZE: usize = 50; // 100x100x100 | (50,50,50) ist maximale Auslenkung


fn main()
{
    let mut capper = FpsCapper::new(FPS);
    let mut frame_buffer: Vec<Vec<char>> = vec![vec![' '; SCREEN_W]; SCREEN_H];
    let mut z_buffer: Vec<Vec<f32>> = vec![vec![0.0 as f32; SCREEN_W]; SCREEN_H]; // use 1/z !

    // define your objects to place here:
    let torus: Torus = Torus::new(10.0, 3.0, vec![0.0, 0.0, 0.0]);

    // main render loop
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout(),cursor::Hide).unwrap(); // hide cursor
    let mut running = true;
    while running
    {
        FpsCapper::start_measurement(&mut capper);
        
        // listen for input
        // change vars according to input
        
        let mut points_to_draw = Torus::get_points_to_draw(&torus, PI / 10.0, PI / 10.0);

        // calculate point rotations
        // filter the points that are out of bounds
        // map the points to the screen grid using buffers + textures + surface normal

        print_screen(&frame_buffer);
        clear_frame_buffer(&mut frame_buffer);
        clear_z_buffer(&mut z_buffer);

        FpsCapper::cap_fps(&mut capper);
        if space_pressed() { running = false; }
    }

    execute!(stdout(), cursor::Show).unwrap(); // reveal cursor
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    pause();
}

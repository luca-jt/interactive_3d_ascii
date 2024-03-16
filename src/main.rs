pub mod geom_funcs;
pub use crate::geom_funcs::*;
pub mod fps_capper;
pub use crate::fps_capper::*;
pub mod geo_objects;
pub use crate::geo_objects::*;


const DISTANCE_FROM_SCREEN: u32 = 10; // change that to your liking
const FPS: u8 = 60;
const TEXTURE: &str = ".-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const SCREEN_W: usize = 64;
const SCREEN_H: usize = 24;
const RENDER_SPACE_SIZE: usize = 50; // 100x100x100


fn main()
{
    let mut capper = FpsCapper::new(FPS);
    let mut frame_buffer: Vec<Vec<char>> = vec![vec![' '; SCREEN_W]; SCREEN_H];
    let mut z_buffer: Vec<Vec<f32>> = vec![vec![0.0 as f32; SCREEN_W]; SCREEN_H]; // use 1/z

    // main render loop
    loop
    {
        FpsCapper::start_measurement(&mut capper);
        print!("\x1b[2J"); // clear screen

        //...

        FpsCapper::cap_fps(&mut capper);
    }
}

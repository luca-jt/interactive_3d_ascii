use std::f32::consts::PI;


pub struct Hedron
{
    coordinates: Vec<Vec<f32>>
}


pub struct Sphere
{
    radius: f32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32
}


pub struct Torus
{
    radius: f32,
    thickness: f32,
    pos: Vec<f32>
}

impl Torus
{
    pub fn new(radius: f32, thickness: f32, pos: Vec<f32>) -> Torus
    {
        Torus { radius, thickness, pos }
    }

    pub fn get_points_to_draw(torus: &Torus, d_theta: f32, d_phi: f32) -> Vec<Vec<f32>>
    {
        let mut points: Vec<Vec<f32>> = Vec::new();
        let mut loop_counter: u32 = 0;
        let mut curr_theta: f32 = 0.0;
        let mut loop_counter2: u32 = 0;
        let mut curr_phi: f32 = 0.0;
        let r1 = torus.radius / 2.0;
        let r2 = torus.thickness / 2.0;

        while loop_counter == 0 || curr_theta != 0.0
        {
            curr_theta = (d_theta * loop_counter as f32) % PI;

            while loop_counter2 == 0 || curr_phi != 0.0
            {
                curr_phi = (d_phi * loop_counter2 as f32) % PI;

                let curr_draw_point: Vec<f32> =
                                    vec![(r2 + r1 * curr_theta.cos()) * curr_phi.cos(), // TODO: add the torus.pos offset
                                    r1 * curr_theta.sin(),
                                    -(r2 + r1 * curr_theta.cos()) * curr_phi.sin()];

                points.push(curr_draw_point);
                loop_counter2 = loop_counter2.wrapping_add(1);
            }
            loop_counter2 = 0;
            loop_counter = loop_counter.wrapping_add(1);
        }
        points
    }
}

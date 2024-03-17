use std::f32::consts::PI;


pub struct Hedron
{
    coordinates: Vec<Vec<f32>>
}

impl Hedron
{
    pub fn new(coordinates: Vec<Vec<f32>>) -> Hedron
    {
        Hedron { coordinates }
    }
}


pub struct Sphere
{
    radius: f32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32
}

impl Sphere
{
    pub fn new(radius: f32, pos_x: f32, pos_y: f32, pos_z: f32) -> Sphere
    {
        Sphere { radius, pos_x, pos_y, pos_z }
    }
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
        let mut outer_loops: u32 = 0;
        let mut theta: f32 = 0.0;
        let mut inner_loops: u32 = 0;
        let mut phi: f32 = 0.0;
        let r1 = torus.radius / 2.0; // just an alias
        let r2 = torus.thickness / 2.0; // "
        let p1 = torus.pos[0]; // offset coords of torus
        let p2 = torus.pos[1];
        let p3 = torus.pos[2];

        while outer_loops == 0 || theta != 0.0
        {
            theta = (d_theta * outer_loops as f32) % PI;

            while inner_loops == 0 || phi != 0.0
            {
                phi = (d_phi * inner_loops as f32) % PI;

                let curr_draw_point: Vec<f32> =
                                    vec![(r2 + r1 * theta.cos() + p1) * phi.cos() - p3 * phi.sin(),
                                    r1 * theta.sin() + p2,
                                    (r2 + r1 * theta.cos() + p1) * phi.sin() + p3 * phi.cos()];

                points.push(curr_draw_point);
                inner_loops = inner_loops.wrapping_add(1);
            }
            inner_loops = 0;
            outer_loops = outer_loops.wrapping_add(1);
        }

        points
    }
}

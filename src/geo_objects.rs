use std::f32::consts::PI;


fn calc_triangle_points(verteces: &Vec<Vec<f32>>, num_of_parts: u32) -> Vec<Vec<f32>>
{
    let mut points: Vec<Vec<f32>> = verteces.clone();

    let start = &points[0];
    // TODO: implement vector arithmetic trait
    // punkte gleichmäßig verteilt -> skizze
    //let slice_width = 

    points
}


pub struct Tetrahedron
{
    coordinates: Vec<Vec<f32>>
}

impl Tetrahedron
{
    pub fn new(coordinates: Vec<Vec<f32>>) -> Tetrahedron
    {
        Tetrahedron { coordinates }
    }

    pub fn get_points_to_draw(hedron: &Tetrahedron, num_of_parts: u32) -> Vec<Vec<f32>>
    {
        let mut points: Vec<Vec<f32>> = Vec::new();
        let coords = hedron.coordinates.clone();

        let triangle1 = vec![coords[0].clone(), coords[1].clone(), coords[2].clone()];
        points.append(&mut calc_triangle_points(&triangle1, num_of_parts));
        let triangle2 = vec![coords[0].clone(), coords[1].clone(), coords[3].clone()];
        points.append(&mut calc_triangle_points(&triangle2, num_of_parts));
        let triangle3 = vec![coords[0].clone(), coords[2].clone(), coords[3].clone()];
        points.append(&mut calc_triangle_points(&triangle3, num_of_parts));
        let triangle4 = vec![coords[1].clone(), coords[2].clone(), coords[2].clone()];
        points.append(&mut calc_triangle_points(&triangle4, num_of_parts));

        points
    }
}


pub struct Sphere
{
    radius: f32,
    px: f32,
    py: f32,
    pz: f32
}

impl Sphere
{
    pub fn new(radius: f32, px: f32, py: f32, pz: f32) -> Sphere
    {
        Sphere { radius, px, py, pz }
    }

    pub fn get_points_to_draw(sphere: &Sphere, d_theta: f32, d_phi: f32) -> Vec<Vec<f32>>
    {
        let mut points: Vec<Vec<f32>> = Vec::new();

        let mut outer_loops: u32 = 0;
        let mut theta: f32 = 0.0;
        let mut inner_loops: u32 = 0;
        let mut phi: f32 = 0.0;

        while outer_loops == 0 || theta != 0.0
        {
            theta = (d_theta * outer_loops as f32) % PI;

            while inner_loops == 0 || phi != 0.0
            {
                phi = (d_phi * inner_loops as f32) % PI;

                let curr_draw_point: Vec<f32> = vec![
                    sphere.px + sphere.radius * theta.cos() * phi.cos(),
                    sphere.py - sphere.radius * phi.sin() * theta.cos(),
                    sphere.pz + sphere.radius * theta.sin()
                ];

                points.push(curr_draw_point);
                inner_loops = inner_loops.wrapping_add(1);
            }
            inner_loops = 0;
            outer_loops = outer_loops.wrapping_add(1);
        }

        points
    }
}


pub struct Torus
{
    radius: f32,
    thickness: f32,
    p1: f32,
    p2: f32,
    p3: f32
}

impl Torus
{
    pub fn new(radius: f32, thickness: f32, p1: f32, p2: f32, p3: f32) -> Torus
    {
        Torus { radius, thickness, p1, p2, p3 }
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

        while outer_loops == 0 || theta != 0.0
        {
            theta = (d_theta * outer_loops as f32) % PI;

            while inner_loops == 0 || phi != 0.0
            {
                phi = (d_phi * inner_loops as f32) % PI;

                let curr_draw_point: Vec<f32> = vec![
                                        (r2 + r1 * theta.cos() + torus.p1) * phi.cos() - torus.p3 * phi.sin(),
                                        r1 * theta.sin() + torus.p2,
                                        (r2 + r1 * theta.cos() + torus.p1) * phi.sin() + torus.p3 * phi.cos()
                                    ];

                points.push(curr_draw_point);
                inner_loops = inner_loops.wrapping_add(1);
            }
            inner_loops = 0;
            outer_loops = outer_loops.wrapping_add(1);
        }

        points
    }
}

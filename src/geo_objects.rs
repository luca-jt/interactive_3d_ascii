use std::f32::consts::PI;

pub use crate::vec_methods::*;


fn calc_triangle_points(verteces: &Vec<Vec<f32>>, num_of_parts: u32) -> Vec<Vec<f32>>
{
    let mut points: Vec<Vec<f32>> = verteces.clone();

    // Edge 1:
    let g1 = &verteces[1].sub(&verteces[0]).unwrap();
    let slice_width1 = g1.norm().unwrap() / num_of_parts as f32;
    // Edge 2:
    let g2 = &verteces[2].sub(&verteces[0]).unwrap();
    let slice_width2 = g2.norm().unwrap() / num_of_parts as f32;

    let mut i = 1;
    while i <= num_of_parts
    {
        let point1 = &verteces[0].add(&g1.normalize().unwrap().mult(i as f32 * slice_width1)).unwrap();
        let point2 = &verteces[0].add(&g2.normalize().unwrap().mult(i as f32 * slice_width2)).unwrap();
        
        // draw qvector from point 1 to point 2
        let qline = point2.sub(point1).unwrap();
        let q_slice_width = qline.norm().unwrap() / (i + 1) as f32;

        // add points p to "points"
        let mut j = 0;
        while j < i + 1
        {
            let p = point1.add(&qline.normalize().unwrap().mult(j as f32 * q_slice_width)).unwrap();
            points.push(p);
            j += 1;
        }
        i += 1;
    }
    points
}


pub struct Tetrahedron
{
    pos1: Vec<f32>,
    pos2: Vec<f32>,
    pos3: Vec<f32>,
    pos4: Vec<f32>
}

impl Tetrahedron
{
    pub fn new(pos1: Vec<f32>, pos2: Vec<f32>, pos3: Vec<f32>, pos4: Vec<f32>) -> Tetrahedron
    {
        Tetrahedron { pos1, pos2, pos3, pos4 }
    }

    pub fn get_points_to_draw(h: &Tetrahedron, num_of_parts: u32) -> Vec<Vec<f32>>
    {
        let mut points: Vec<Vec<f32>> = Vec::new();

        let triangle1 = vec![h.pos1.clone(), h.pos2.clone(), h.pos3.clone()];
        points.append(&mut calc_triangle_points(&triangle1, num_of_parts));
        let triangle2 = vec![h.pos1.clone(), h.pos2.clone(), h.pos4.clone()];
        points.append(&mut calc_triangle_points(&triangle2, num_of_parts));
        let triangle3 = vec![h.pos1.clone(), h.pos3.clone(), h.pos4.clone()];
        points.append(&mut calc_triangle_points(&triangle3, num_of_parts));
        let triangle4 = vec![h.pos2.clone(), h.pos3.clone(), h.pos4.clone()];
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

        while theta < 2.0 * PI
        {
            while phi < 2.0 * PI
            {
                let curr_draw_point: Vec<f32> = vec![
                    sphere.px + sphere.radius * theta.cos() * phi.cos(),
                    sphere.py - sphere.radius * phi.sin() * theta.cos(),
                    sphere.pz + sphere.radius * theta.sin()
                ];

                points.push(curr_draw_point);
                inner_loops = inner_loops.wrapping_add(1);

                phi = d_phi * inner_loops as f32;
            }
            inner_loops = 0;
            phi = 0.0;

            outer_loops = outer_loops.wrapping_add(1);
            theta = d_theta * outer_loops as f32;
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

        let r1 = torus.thickness / 2.0; // just aliases
        let r2 = torus.radius / 2.0;

        while theta < 2.0 * PI
        {
            while phi < 2.0 * PI
            {
                let curr_draw_point: Vec<f32> = vec![
                                        (r2 + r1 * theta.cos() + torus.p1) * phi.cos() - torus.p3 * phi.sin(),
                                        r1 * theta.sin() + torus.p2,
                                        (r2 + r1 * theta.cos() + torus.p1) * phi.sin() + torus.p3 * phi.cos()
                                    ];

                points.push(curr_draw_point);
                inner_loops = inner_loops.wrapping_add(1);

                phi = d_phi * inner_loops as f32;
            }
            inner_loops = 0;
            phi = 0.0;

            outer_loops = outer_loops.wrapping_add(1);
            theta = d_theta * outer_loops as f32;
        }

        points
    }
}

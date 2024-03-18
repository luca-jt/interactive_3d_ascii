
fn calc_x_pos(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32
{
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos() +
    j * a.cos() * c.sin() + k * a.sin() * c.sin() + i * b.cos() * c.cos()
}

fn calc_y_pos(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32
{
    j * a.cos() * c.cos() + k * a.sin() * c.cos() -
    j * a.sin() * b.sin() * c.sin() + k * a.cos() * b.sin() * c.sin() -
    i * b.cos() * c.sin()
}

fn calc_z_pos(i: f32, j: f32, k: f32, a: f32, b: f32) -> f32
{
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}


pub fn calc_rotation(ps: &Vec<Vec<f32>>, a:f32, b: f32, c: f32) -> Vec<Vec<f32>>
{
    let mut result: Vec<Vec<f32>> = Vec::new();

    for p in ps
    {
        result.push(vec![
            calc_x_pos(p[0], p[1], p[2], a, b, c),
            calc_y_pos(p[0], p[1], p[2], a, b, c),
            calc_z_pos(p[0], p[1], p[2], a, b)
        ])
    }

    result
}

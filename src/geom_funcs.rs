
pub fn calc_x_pos(i: i32, j: i32, k: i32, a: f32, b: f32, c: f32) -> f32
{
    j as f32 * a.sin() * b.sin() * c.cos() - k as f32 * a.cos() * b.sin() * c.cos() +
    j as f32 * a.cos() * c.sin() + k as f32 * a.sin() * c.sin() + i as f32 * b.cos() * c.cos()
}

pub fn calc_y_pos(i: i32, j: i32, k: i32, a: f32, b: f32, c: f32) -> f32
{
    j as f32 * a.cos() * c.cos() + k as f32 * a.sin() * c.cos() -
    j as f32 * a.sin() * b.sin() * c.sin() + k as f32 * a.cos() * b.sin() * c.sin() -
    i as f32 * b.cos() * c.sin()
}

pub fn calc_z_pos(i: i32, j: i32, k: i32, a: f32, b: f32) -> f32
{
    k as f32 * a.cos() * b.cos() - j as f32 * a.sin() * b.cos() + i as f32 * b.sin()
}

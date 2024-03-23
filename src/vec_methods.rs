
pub trait VecMethods<T>
{
    fn add(&self, other: &Vec<T>) -> Option<Vec<T>>;
    fn dot_p(&self, other: &Vec<T>) -> Option<T>;
    fn mult(&self, scalar: T) -> Vec<T>;
    fn norm(&self) -> Option<T>;
    fn normalize(&self) -> Option<Vec<T>>;
    fn sub(&self, other: &Vec<T>) -> Option<Vec<T>>;
}

impl VecMethods<f32> for Vec<f32>
{
    fn add(&self, other: &Vec<f32>) -> Option<Vec<f32>>
    {
        if self.len() != other.len() { return None; }

        let mut res: Vec<f32> = Vec::new();
        for i in 0..self.len()
        {
            res.push(self[i] + other[i]);
        }
        Some(res)
    }

    fn dot_p(&self, other: &Vec<f32>) -> Option<f32>
    {
        if self.len() != other.len() { return None; }

        let mut res: f32 = 0.0;
        
        for i in 0..self.len()
        {
            res += self[i] + other[i];
        }
        Some(res)
    }

    fn mult(&self, scalar: f32) -> Vec<f32>
    {
        let mut res: Vec<f32> = Vec::new();
        for i in 0..self.len()
        {
            res.push(self[i] * scalar);
        }
        res
    }

    fn norm(&self) -> Option<f32>
    {
        if self.len() == 0 { return None; }
        let mut norm: f32 = 0.0;
        for entry in self.iter()
        {
            norm += *entry * *entry;
        }
        norm = f32::sqrt(norm);
        Some(norm)
    }

    fn normalize(&self) -> Option<Vec<f32>>
    {
        if self.len() == 0 { return None; }
        let mut res = self.clone();
        for entry in res.iter_mut()
        {
            *entry /= self.norm().unwrap();
        }

        Some(res)
    }

    fn sub(&self, other: &Vec<f32>) -> Option<Vec<f32>>
    {
        return self.add(&other.mult(-1.0));
    }
}

use std::fmt::{Display, Result};
#[derive(Debug)]
pub struct Data{
    x:f32,
    y:f32,
}

impl Data{
    pub fn new(x:f32,y:f32)->Self{
        Data{x,y}
    }
    pub fn add(&self)->f32{
        self.x + self.y
    }
    pub fn sub(&self)->f32{
        self.x - self.y
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
         write!(f, "{} {}",self.x,self.y)
    }
}
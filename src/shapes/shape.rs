pub struct Rectangle{
    pub width:f32,
    pub height:f32,
}
pub struct Triangle{
    pub width:f32,
    pub height:f32,
}
pub trait Shape {
    fn new (width:f32,height:f32)->Self;
    fn get_shape(&self)-> (f32,f32);
}

impl Rectangle {
    pub fn area(&self)->f32{
        self.width * self.height
    }
}
impl Triangle{
    pub fn area(&self)->f32{
        self.width * self.height * 0.5
    }
}

impl Shape for Rectangle {
    fn new(width:f32,height:f32)->Self{
        Rectangle{width,height}
    }
    fn get_shape(&self)->(f32,f32){
        (self.width,self.height)
    }
}

impl Shape for Triangle {
    fn new(width:f32,height:f32)->Self{
        Triangle{width,height}
    }
    fn get_shape(&self)-> (f32,f32){
        (self.width,self.height)
    }
}
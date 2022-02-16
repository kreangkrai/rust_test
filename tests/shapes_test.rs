use rust_test::shapes::shape::*;
#[test]
fn rec_test(){
    let rec = Rectangle::new(5.0,4.0);
    assert_eq!(rec.area(),20.0);   
}
#[test]
fn triangle_test(){
    let triangle = Triangle::new(5.0, 4.0);
    assert_eq!(triangle.area(),10.0);
}
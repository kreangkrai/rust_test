use rust_test::calculates::*;
use rust_test::shapes::shape::*;
use rust_test::files::file::{read_file};
fn main(){

    let s = read_file();
    match s {
        Ok(s) =>{
            println!("{:?}",s);
        },
        Err(e) =>{
            println!("{:?}",e);
        }
    }

    let data = Data::new(5.1, 4.5);
    let add = data.add();
    let sub = data.sub();

    println!("{:?}",data);
    println!("add : {}",add);
    println!("sub : {}",sub);

    let rec1 = Rectangle::new(2.0,3.2);
    let shape = rec1.get_shape();
    let area = rec1.area();

    println!("Rectangle : {},{} Area: {}",shape.0,shape.1, area);
}



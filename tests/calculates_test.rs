use rust_test::calculates::*;
#[test]
fn cal_test(){
    let data = Data::new(5.0,4.0);
    assert_eq!(data.add(),9.0);
    assert_eq!(data.sub(),1.0);
}
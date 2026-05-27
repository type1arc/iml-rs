use iml_rust::constants::constant;
use iml_rust::linear_alg::{self, Cross, Dot, Vec3};
use iml_rust::log;
use iml_rust::trig;

fn main() {
    log!(trig::sin(0.0000001));
    log!(trig::cos(constant::PI));
    log!(trig::tan(constant::PI / 4.0));

    let v1 = Vec3::new(1, 3, 7);
    let v2 = Vec3::new(-1, 4, -9);

    let sum = v1 + v2;
    sum.logtty();

    // Also test the generic Vector
    let gv1 = linear_alg::Vector::new([1.0, 3.0, 7.0]);
    let gv2 = linear_alg::Vector::new([-1.0, 3.0, 4.0]);
    let _gsum = &gv1 + &gv2;
    let _gdp = gv1.dot(&gv2);
    let _gcrp = gv1.cross_product(&gv2);
    println!("{:?}", _gcrp);
}

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::vector;
use na::{Translation, UnitQuaternion, Vector3};

fn main() {
    env_logger::init();
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(0.1, 0.1, 0.1);

    c.set_color(0.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let rot_1 = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -0.5);
    c.append_rotation(&rot_1);
    
    //c.append_translation(&Translation{ vector: vector!(0.05, 0.05, 0.05) });


    let mut c2 = window.add_cube(0.1, 0.1, 0.1);
    c2.set_color(0.0, 0.0, 1.0);
    c2.append_translation(&Translation{ vector: vector!(0.1, 0.1, 0.1) });
    c2.append_rotation(&rot_1);

    while window.render() {
        c.append_rotation(&rot);
        c2.append_rotation(&rot);
    }
}
extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::vector;
use na::{Translation, UnitQuaternion, Vector3};

fn main() {
    env_logger::init();
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut cubes = Vec::new();
    let rot1 = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -0.5);
    for i in 0..27 {
        let c = window.add_cube(0.08, 0.08, 0.08);
        cubes.push(c);

        cubes[i].append_rotation(&rot1);

        let r = (i % 3) as f32 * 0.1;
        let c = (i / 9) as f32 * 0.1;
        let d = (i % 9) as f32 * 0.05;
        cubes[i].append_translation(&Translation { vector: vector!(r, c, d) });
    }

    for i in 0..9 {
        cubes[i].set_color(1., 0., 0.);
    }
    for i in 9..27 {
        cubes[i].set_color(0., 1., 0.);
    }
    for i in 18..27 {
        cubes[i].set_color(0., 0., 1.);
    }

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    while window.render() {
        for i in 0..27 {
            cubes[i].append_rotation(&rot);
        }
    }
}
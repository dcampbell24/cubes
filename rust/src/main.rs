extern crate clap;
extern crate kiss3d;
extern crate nalgebra as na;

use clap::Parser;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::vector;
use na::{Translation, UnitQuaternion, Vector3};

/// Program to display a 3x3 cube solution.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Solve the minotaur problem.
   #[arg(short, long)]
   minotaur: bool,

   /// Solve the planes problem.
   #[arg(short, long)]
   planes: bool,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);

    let mut cubes = Vec::new();
    //let rot1 = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -0.5);
    for i in 0..27 {
        let c = window.add_cube(0.08, 0.08, 0.08);
        cubes.push(c);

        //cubes[i].append_rotation(&rot1);

        let r = (i % 3) as f32 * 0.1;
        let c = (i / 9) as f32 * 0.1;
        let d = (i % 9) as f32 * 0.05;
        cubes[i].append_translation(&Translation { vector: vector!(r, c, d) });
    }

    if args.minotaur {
        cubes[0].set_color(1.0, 0.0, 0.0);
        cubes[1].set_color(1.0, 0.0, 0.0);
        cubes[2].set_color(1.0, 0.0, 0.0);

        cubes[3].set_color(1.0, 0.0, 0.0);
        cubes[4].set_color(0.0, 1.0, 0.0);
        cubes[5].set_color(0.0, 0.0, 1.0);

        cubes[6].set_color(0.0, 1.0, 0.0);
        cubes[7].set_color(0.0, 1.0, 0.0);
        cubes[8].set_color(0.0, 0.0, 1.0);


        cubes[9].set_color(1.0, 1.0, 0.0);
        cubes[10].set_color(1.0, 0.0, 0.0);
        cubes[11].set_color(0.0, 1.0, 1.0);

        cubes[12].set_color(1.0, 1.0, 0.0);
        cubes[13].set_color(1.0, 1.0, 0.0);
        cubes[14].set_color(0.0, 1.0, 1.0);

        cubes[15].set_color(0.0, 1.0, 0.0);
        cubes[16].set_color(0.0, 0.0, 1.0);
        cubes[17].set_color(0.0, 0.0, 1.0);


        cubes[18].set_color(1.0, 0.0, 1.0);
        cubes[19].set_color(1.0, 0.0, 1.0);
        cubes[20].set_color(1.0, 0.0, 1.0);

        cubes[21].set_color(1.0, 1.0, 0.0);
        cubes[22].set_color(1.0, 0.0, 1.0);
        cubes[23].set_color(0.0, 1.0, 1.0);

        cubes[24].set_color(1.0, 1.0, 0.0);
        cubes[25].set_color(0.0, 1.0, 1.0);
        cubes[26].set_color(0.0, 1.0, 1.0);
    }

    if args.planes {
        for i in 0..9 {
            cubes[i].set_color(1., 0., 0.);
        }
        for i in 9..27 {
            cubes[i].set_color(0., 1., 0.);
        }
        for i in 18..27 {
            cubes[i].set_color(0., 0., 1.);
        }
    }

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    while window.render() {
        for i in 0..27 {
            cubes[i].append_rotation(&rot);
        }
    }
}
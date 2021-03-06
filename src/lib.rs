#![allow(dead_code, unused_must_use)]
extern crate core;

mod canvas;
mod color;
mod matrix;
mod point;
mod vector;

use crate::point::Point;
use crate::vector::Vector;

pub fn equal(f1: f64, f2: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    if (f1 - f2).abs() < EPSILON {
        return true;
    }
    false
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::canvas::Canvas;
    use crate::color::Color;
    use crate::point::Point;

    #[test]
    fn fire_virtual_cannon() {
        let start = Point::new(0.0, 1.0, 0.0);
        let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
        let mut projectile = Projectile::new(start, velocity);

        let gravity = Vector::new(0.0, -0.1, 0.0);
        let wind = Vector::new(-0.01, 0.0, 0.0);

        let environment = Environment { gravity, wind };

        let mut canvas = Canvas::new(900, 550);

        println!();
        let mut i = 0;
        while projectile.position.y > 0.0 {
            tick(&environment, &mut projectile);

            let (x, y) = (
                projectile.position.x as usize,
                canvas.height - projectile.position.y as usize,
            );
            let red = Color::new(1.0, 0.0, 0.0);

            canvas.set_pixel(x, y, red);

            i += 1;
        }
        println!();
        println!("Cannonball went {:#?} meters!", projectile.position.x);
        println!("Finished after {} ticks", i);

        canvas.write_to_file().expect("Could not write to file")
    }
}

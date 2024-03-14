#[macro_use]
extern crate bmp;

mod vector;

use crate::vector::Vector;
use std::io::Write;
use bmp::{Image, Pixel};

struct Sphere {
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
}


fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 400;

    let s = Sphere {
        x: 0.0,
        y: 0.0,
        z: 20.0,
        radius: 3.0,
    };

    let width = WIDTH as f64;
    let height = HEIGHT as f64;
    let camera_position = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let camera_direction = Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    // let mut screen = [[[0.0; WIDTH]; HEIGHT]; 3]; // X*Y*3

    let mut image = Image::new(WIDTH as u32, HEIGHT as u32);
    //    println!("woo {}", screen[3][0][0]);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let x_direction = (x as f64 * 6.0) / width - 3.0;
            let y_direction = (y as f64 * 6.0) * height / width / height - 3.0 * height / width;

            let direction = Vector {
                x: x_direction / 6.0,
                y: y_direction / 6.0,
                z: 1.0,
            }
                .normalize();
            let res = ray_sphere_intersection(&camera_position, &direction, &s);

            match res {
                None => {
                    image.set_pixel(x as u32, y as u32, px!(0,0,0))
                }
                Some(_) => {
                    image.set_pixel(x as u32, y as u32, px!(255,255,255));
                }
            }
        }
    }


    let _ = image.save("result.bmp");
}

fn ray_sphere_intersection(start: &Vector, direction: &Vector, sphere: &Sphere) -> Option<Vector> {
    let center = Vector {
        x: sphere.x,
        y: sphere.y,
        z: sphere.z,
    };

    let v = start.minus(&center);

    let wee = (v.dot(direction)) * (v.dot(direction))
        - (v.x * v.x + v.y * v.y + v.z * v.z - sphere.radius * sphere.radius);

    if wee <= 0.0 {
        return None;
    }

    let intersection1 = v.dot(direction) * -1.0 + f64::sqrt(wee);
    let intersection2 = v.dot(direction) * -1.0 - f64::sqrt(wee);

    let closest_intersection = if intersection1 < intersection2 && intersection1 > 0.0001 {
        intersection1
    } else if intersection2 < intersection1 && intersection2 > 0.0001 {
        intersection2
    } else {
        return None;
    };

    let end_distance = direction.multiply(closest_intersection);
    let end_position = start.add(&end_distance);
    Some(end_position)
}

/*
override fun getIntersection(start: Vector, direction: Vector): SurfacePoint? {
        val center  = this.position
        val v = start - center

        val wee=(v.dot(direction))*(v.dot(direction))-(v.x*v.x+v.y*v.y+v.z*v.z-this.radius*this.radius)
        if(wee > 0) {
            val intersectionDistance = arrayOf(v.dot(direction)*-1+sqrt(wee),
                v.dot(direction)*-1-sqrt(wee))

            val intersectionsInDirection = intersectionDistance.filter { it > 0.00001 }

            val closestIntersection = intersectionsInDirection.minOrNull() ?: return null

            val endDistance = direction * closestIntersection
            val endPosition = start + endDistance

            return SurfacePoint(endPosition, getNormal(endPosition), this.material)
        }
        return null
    }


 */

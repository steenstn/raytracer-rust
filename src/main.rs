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
    color: Color,
    light: bool,
}


struct SurfacePoint {
    position: Vector,
    normal: Vector,
    color: Color,
}

struct Color(f64, f64, f64);

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

const MAX_BOUNCES: u32 = 20;
const NUM_RAYS: u32 = 200;

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 400;

    let spheres = [Sphere {
        x: -5.0,
        y: 0.0,
        z: 40.0,
        radius: 2.8,
        color: Color(1.0, 0.2, 0.3),
        light: false,
    }, Sphere {
        x: 5.0,
        y: 0.0,
        z: 30.0,
        radius: 3.0,
        color: Color(10.0, 10.0, 10.0),
        light: true,
    },
        Sphere {
            x: 4.0,
            y: 8.0,
            z: 30.0,
            radius: 2.0,
            color: Color(0.1, 0.4, 0.65),
            light: false,
        }];

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


    let mut image = Image::new(WIDTH as u32, HEIGHT as u32);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let x_direction = (x as f64 * 6.0) / width - 3.0;
            let y_direction = (y as f64 * 6.0) / width - 3.0 * height / width;

            let direction = Vector {
                x: x_direction / 6.0,
                y: y_direction / 6.0,
                z: 1.0,
            }.normalize();
            let mut res = Color(0.0, 0.0, 0.0);

            for i in 0..NUM_RAYS {
                let new_color = shoot_ray(&camera_position, &direction, &spheres, 0);
                res.0 += new_color.0;
                res.1 += new_color.1;
                res.2 += new_color.2;
            }

            res.0 /= NUM_RAYS as f64;
            res.1 /= NUM_RAYS as f64;
            res.2 /= NUM_RAYS as f64;

            image.set_pixel(x as u32, y as u32, px!(res.0*255.0,res.1*255.0,res.2*255.0))
        }
    }


    let _ = image.save("result.bmp");
}

fn shoot_ray(start: &Vector, direction: &Vector, spheres: &[Sphere], num_bounces: u32) -> Color {
    if num_bounces > MAX_BOUNCES {
        return Color(0.0, 0.0, 0.0);
    }
    let mut distance = 99999999.0;
    let mut hit_point: Option<SurfacePoint> = None;
    let mut hit_sphere: Option<&Sphere> = None;
    for s in spheres {
        match ray_sphere_intersection(&start, &direction, s) {
            None => {}
            Some(surface_point) => {
                let length = surface_point.position.minus(start).length();
                if length < distance {
                    distance = length
                }
                hit_point = Some(surface_point);
                hit_sphere = Some(s);
            }
        }
    }

    return match hit_point {
        None => {
            Color(0.0, 0.0, 0.0)
        }
        Some(sp) => {
            if (hit_sphere.unwrap().light == true) {
                return Color(hit_sphere.unwrap().color.0, hit_sphere.unwrap().color.1, hit_sphere.unwrap().color.2);
            }
            let random_vector = vector::random_vector();
            let crossed = random_vector.cross(&sp.normal).normalize();
            let eps1 = rand::random::<f64>() * std::f64::consts::PI * 2.0;
            let eps2 = f64::sqrt(rand::random::<f64>());

            let x = f64::cos(eps1) * eps2;
            let y = f64::sin(eps1) * eps2;
            let z = f64::sqrt(1.0 - eps2 * eps2);

            let tangent = sp.normal.cross(&crossed);

            let new_direction: Vector = crossed * x + &(&tangent * y) + &(&sp.normal * z);
            let reflected = shoot_ray(&sp.position, &new_direction.normalize(), &spheres, num_bounces + 1);

            Color(sp.color.0 * reflected.0, sp.color.1 * reflected.1, sp.color.2 * reflected.2)
        }
    };
}

fn ray_sphere_intersection(start: &Vector, direction: &Vector, sphere: &Sphere) -> Option<SurfacePoint> {
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
    let sphere_position = Vector { x: sphere.x, y: sphere.y, z: sphere.z };
    let normal = end_position.minus(&sphere_position).normalize();

    Some(SurfacePoint { position: end_position, normal, color: Color(sphere.color.0, sphere.color.1, sphere.color.2) })
}

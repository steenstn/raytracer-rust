#[macro_use]
extern crate bmp;

mod vector;

use crate::vector::Vector;
use std::io::Write;
use bmp::{Image, Pixel};
use multiarray::*;

struct Sphere {
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
    color: Color,
    light: bool,
}

struct Plane {
    x: f64,
    y: f64,
    z: f64,
    normal: Vector,
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
const NUM_RAYS: u32 = 50;

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;

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
        },
    ];
    let p = Plane {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        normal: Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
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

    let mut image = Array3D::new([WIDTH, HEIGHT, 3], 0.0);
    let mut bmp_image = Image::new(WIDTH as u32, HEIGHT as u32);
    let mut num_passes: u32 = 1;
    loop {
        for x in 0..WIDTH {
            if (x % 10 == 0) {
                println!("{x}/{WIDTH}");
            }
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

                image[[x, y, 0]] += res.0 ;
                image[[x, y, 1]] += res.1 ;
                image[[x, y, 2]] += res.2 ;

                bmp_image.set_pixel(x as u32, y as u32,
                                    px!(clamp(image[[x, y, 0]]/ num_passes as f64 * 255.0, 0.0, 255.0),
                    clamp(image[[x, y, 1]]/ num_passes as f64 * 255.0, 0.0, 255.0),
                    clamp(image[[x, y, 2]]/ num_passes as f64 * 255.0, 0.0,255.0)))
            }
        }


        num_passes += 1;
        let _ = bmp_image.save("result.bmp");
    }
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
            if hit_sphere.unwrap().light == true {
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


fn ray_plane_intersection(start: &Vector, direction: &Vector, plane: &Plane) -> Option<SurfacePoint> {
    let plane_position = Vector { x: plane.x, y: plane.y, z: plane.z };
    let distance = plane_position.minus(start).dot(&plane.normal) / (direction.dot(&plane.normal));
    if distance > 0.00001 {
        let end_movement = direction * distance;
        let intersection_point = start.add(&end_movement);
        return Some(SurfacePoint { position: intersection_point, normal: Vector { x: plane.normal.x, y: plane.normal.y, z: plane.normal.z }, color: Color(0.7, 0.7, 0.7) });
    }
    return None;
    /*
          val distance = (position-start).dot(normal)/direction.dot(normal)
        if(distance > 0.00001) {
            val endMovement = direction * distance
            val intersectionPoint = start + endMovement
            return SurfacePoint(intersectionPoint, normal, material)
        }
        return null
    }
     */
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

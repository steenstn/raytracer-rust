use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn dot(&self, b: &Vector) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    fn length(&self) -> f64 {
        return f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    // Check if correct
    fn cross(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.y * b.z - self.z * b.y,
            y: -1.0 * (self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        };
    }
    fn add(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        };
    }

    fn minus(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        };
    }

    fn multiply(&self, value: f64) -> Vector {
        return Vector {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        };
    }
}


struct Sphere {
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
}

fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    let mut end_image = [0.5; WIDTH * HEIGHT * 3];

    let s = Sphere {
        x: 0.0,
        y: 0.0,
        z: 10.0,
        radius: 3.0,
    };

    let res = ray_sphere_intersection(&Vector { x: 0.0, y: 0.0, z: 0.0 }, &Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }, &s);

    match res {
        None => { println!("MISS") }
        Some(vector) => { println!("Hit at {:?}", vector) }
    }
    write_ppm_file(WIDTH, HEIGHT, &end_image).expect("Fail");
}

fn ray_sphere_intersection(start: &Vector, direction: &Vector, sphere: &Sphere) -> Option<Vector> {
    let center = Vector {
        x: sphere.x,
        y: sphere.y,
        z: sphere.z,
    };

    let v = start.minus(&center);

    let wee = (v.dot(direction)) * (v.dot(direction)) - (v.x * v.x + v.y * v.y + v.z * v.z - sphere.radius * sphere.radius);

    if wee <= 0.0 {
        return None;
    }

    let intersection1 = v.dot(direction) * -1.0 + f64::sqrt(wee);
    let intersection2 = v.dot(direction) * -1.0 - f64::sqrt(wee);


    // Check or fix
    let closest_intersection = if intersection1 < intersection2 && intersection1 > 0.0001 {
        intersection1
    } else if intersection2 < intersection1 && intersection2 > 0.0001 { intersection2 } else {
        return None;
    };

    let end_distance = direction.multiply(closest_intersection);
    let end_position = start.add(&end_distance);
    return Some(end_position);
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
fn write_ppm_file(width: usize, height: usize, image: &[f64]) -> Result<(), io::Error> {
    let mut file = File::create("result.ppm")?;

    file.write(b"P3\n")?;
    write!(file, "{} {}\n", width, height)?;
    file.write(b"255\n")?;
    for e in image {
        write!(file, "{} ", (e * 255.0).round())?;
    }
    write!(file, "\n")?;
    Ok(())
}

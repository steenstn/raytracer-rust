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
            x: self.y * b.z - self.z,
            y: -1.0 * (self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        };
    }
    fn add(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z
        }
    }

    fn minus(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z
        }
    }

}


struct Sphere {
    x: f64,
    y: f64,
    z: f64,
    radius: f64
}

fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    let mut end_image = [0.5; WIDTH * HEIGHT * 3];

    write_ppm_file(WIDTH, HEIGHT, &end_image).expect("Fail");
}



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

use std::ops;
use rand::random;

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Vector {
    pub fn dot(&self, b: &Vector) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn length(&self) -> f64 {
        return f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    pub fn normalize(&self) -> Vector {
        let abs = f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        return Vector { x: self.x / abs, y: self.y / abs, z: self.z / abs };
    }

    // Check if correct
    pub fn cross(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.y * b.z - self.z * b.y,
            y: -1.0 * (self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        };
    }
    pub fn add(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        };
    }

    pub fn minus(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        };
    }

    pub fn multiply(&self, value: f64) -> Vector {
        return Vector {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        };
    }
}

pub(crate) fn random_vector() -> Vector {
    Vector { x: random::<f64>(), y: random::<f64>(), z: random::<f64>() }.normalize()
}

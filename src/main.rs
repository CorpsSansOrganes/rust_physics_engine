use std::ops::MulAssign; // *= overloading

type Real = f32; // Accuracy vs. Percision tradeoff.

struct Vector3 {
    x : Real,
    y : Real,
    z : Real,
}

impl Vector3 {
    pub fn new(x: Real, y: Real, z: Real) -> Vector3 {
        Vector3 {x, y, z}
    }
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    pub fn magnitude(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn normalize(&mut self) {
        *self *= 1.0 / self.magnitude();
    }
}

impl MulAssign<Real> for Vector3 {
    fn mul_assign(&mut self, rhs: Real) {
       self.x *= rhs; 
       self.y *= rhs; 
       self.z *= rhs; 
    }
}

fn main() {
    let v = Vector3::new(1.0, 1.0, 1.0);
    println!("{}, {}, {}", v.x, v.y, v.z);
}

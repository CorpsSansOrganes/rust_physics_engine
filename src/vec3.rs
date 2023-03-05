use std::ops::MulAssign; // *= overloading
use std::ops::Mul; // * overloading
use std::ops::AddAssign; // += overloading                   
use std::ops::Add; // + overloading                   
use std::ops::SubAssign; // -= overloading
use std::ops::Sub; // - overloading

type Real = f32; // Accuracy vs. Percision tradeoff.

pub struct Vector3 {
    pub x : Real,
    pub y : Real,
    pub z : Real,
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
        // Maybe should handle magnitude < 1?
        *self *= 1.0 / self.magnitude();
    }
}

impl MulAssign<Real> for Vector3 {
    fn mul_assign(&mut self, rhs: Real) {
       self.x *= rhs; 
       self.y *= rhs; 
       self.z *= rhs; 
       // *self = *self * rhs; // todo()!
    }
}

impl Mul<Real> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Real) -> Vector3 {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vector3::new(x, y, z)
    }    
}

impl AddAssign for Vector3 {
   fn add_assign(&mut self, rhs: Self) {
       self.x += rhs.x; 
       self.y += rhs.y; 
       self.z += rhs.z; 
   } 
}
impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
       let x = self.x + rhs.x; 
       let y = self.y + rhs.y; 
       let z = self.z + rhs.z; 
       Vector3::new(x, y, z)
    }    
}

impl SubAssign for Vector3 {
   fn sub_assign(&mut self, rhs: Self) {
       self.x -= rhs.x; 
       self.y -= rhs.y; 
       self.z -= rhs.z; 
   } 
}
impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
       let x = self.x - rhs.x; 
       let y = self.y - rhs.y; 
       let z = self.z - rhs.z; 
       Vector3::new(x, y, z)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_regular() {
        let mut v = Vector3::new(1.0, -1.5, 1.111);
        v.invert();
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, 1.5);
        assert_eq!(v.z, -1.111);
    }

    #[test]
    fn invert_zero() {
        let mut v = Vector3::new(0.0, 0.0, 0.0);
        v.invert();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn magnitude() {
        let v = Vector3::new(1.2, 1.25, 1.4);
        let res = 2.2276669409945464286979267648430872869963948352545716000891;
        let acceptable_error = 0.0001;

        assert!(v.magnitude() - res < acceptable_error);
    }

    #[test]
    fn normalize_already_normalized() {
        let f : Real = 3.0;
        let x = 1.0 / f.sqrt();
        let mut v = Vector3::new(x, x, x);
        let acceptable_error = 0.0001;
        v.normalize();

        assert!(v.x - x < acceptable_error);
        assert!(v.y - x < acceptable_error);
        assert!(v.z - x < acceptable_error);
    }

    #[test]
    fn arithmetics() {
        let x = 1.0;
        let v1 = Vector3::new(x, x, x);
        let v2 = Vector3::new(x, -x, x);
        let mut res = v1 + v2;

        assert_eq!(res.x, 2.0);
        assert_eq!(res.y, 0.0);
        assert_eq!(res.z, 2.0);

        res += v1;
        assert_eq!(res.x, 3.0);
        assert_eq!(res.y, 1.0);
        assert_eq!(res.z, 3.0);
    }
}

use std::ops::MulAssign; // *= overloading
use std::ops::Mul; // * overloading
use std::ops::AddAssign; // += overloading                   
use std::ops::Add; // + overloading                   
use std::ops::SubAssign; // -= overloading
use std::ops::Sub; // - overloading
use std::ops::DivAssign; // /= overloading
use std::ops::Div; // / overloading
pub type Real = f32; // Accuracy vs. Percision tradeoff.

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
    pub fn dot_product(&self, rhs: &Vector3) -> Real {
        /*
         * Dot product, aka inner product and scalar product.
         */
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross_product(&self, rhs: &Vector3) -> Vector3 {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = -self.x * rhs.z + self.z * rhs.x;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vector3::new(x, y, z)
    }
}

impl Copy for Vector3 { }

impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        *self
    }
}

impl MulAssign<Real> for Vector3 {
    fn mul_assign(&mut self, rhs: Real) {
       // self.x *= rhs; 
       // self.y *= rhs; 
       // self.z *= rhs; 
       *self = *self * rhs; 
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

impl DivAssign<Real> for Vector3 {
    fn div_assign(&mut self, rhs: Real) {
        *self = *self / rhs
    }
}

impl Div<Real> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Real) -> Self::Output {
       let x = self.x / rhs;
       let y = self.y / rhs;
       let z = self.z / rhs;
       Vector3::new(x, y, z)
    }
}

impl AddAssign for Vector3 {
   fn add_assign(&mut self, rhs: Self) {
       // self.x += rhs.x; 
       // self.y += rhs.y; 
       // self.z += rhs.z; 
       *self = *self + rhs;
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
       // self.x -= rhs.x; 
       // self.y -= rhs.y; 
       // self.z -= rhs.z; 
       *self = *self - rhs;
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

        res /= 2.0;
        assert_eq!(res.x, 1.5);
        assert_eq!(res.y, 0.5);
        assert_eq!(res.z, 1.5);

        res *= -1.0;
        assert_eq!(res.x, -1.5);
        assert_eq!(res.y, -0.5);
        assert_eq!(res.z, -1.5);
    }

    #[test]
    fn dot_product_test() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        // v1 & v2 are perpendicular to each other, therefore their dot 
        // product should be 0.
        assert_eq!(v1.dot_product(&v2), 0.0);

        let v1 = Vector3::new(1.0, 1.0, 0.0);
        let v2 = Vector3::new(3.0, 2.3, 0.0);
        // v1 & v2 point in the same direction, therefore their dot product 
        // should be positive.
        assert!(v1.dot_product(&v2) > 0.0);
        assert_eq!(v1.dot_product(&v2), 5.3);

        let v1 = Vector3::new(1.0, 1.0, 0.0);
        let v2 = Vector3::new(-3.0, -2.3, 3.0);
        // v1 & v2 point in opposite directions, therefore their dot product 
        // should be negative.
        assert!(v1.dot_product(&v2) < 0.0);
        assert_eq!(v1.dot_product(&v2), -5.3);
    }

    #[test]
    fn cross_product_test() {
        let v1 = Vector3::new(2.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 2.0, 0.0);

        // The area of the parallelogram formed by v1 & v2 is 4, which should
        // be the magnitude of their cross product.
        assert_eq!(v1.cross_product(&v2).magnitude(), 4.0);

        let v1 = Vector3::new(-1.0, 3.2, 4.0);
        let v2 = Vector3::new(-2.2, 1.0, -0.5);
        let res = v1.cross_product(&v2);

        // The cross product should be perpendicular to both v1 and v2. 
        // Therefore, the dot product should be 0 for both v1 and v2.
        let acceptable_error = 0.00001;
        assert!(v1.dot_product(&res) < acceptable_error);
        assert!(v2.dot_product(&res) < acceptable_error);

        // Assert we got the expected result (and not the left-hand vector)
        assert!(res.x - -5.6 < acceptable_error);
        assert!(res.y - -9.3 < acceptable_error);
        assert!(res.z - 6.04 < acceptable_error);
    }
}

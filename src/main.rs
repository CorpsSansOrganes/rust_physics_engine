type Real = f32;

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
}

fn main() {
    let v = Vector3::new(1.0, 1.0, 1.0);
    println!("{}, {}, {}", v.x, v.y, v.z);
}

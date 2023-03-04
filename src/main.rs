mod vec3;
use vec3::Vector3;

fn main() {
    let mut v = Vector3::new(1.0, 1.0, 1.0);
    println!("{}, {}, {}", v.x, v.y, v.z);

    v *= 3.0;
}

mod vec3;
mod particle;
use vec3::Vector3;

fn main() {
    let mut v = Vector3::new(1.0, 1.0, 1.0);
    println!("{}, {}, {}", v.x, v.y, v.z);

    v.invert();
    println!("{}, {}, {}", v.x, v.y, v.z);

    v.normalize();
    println!("{}, {}, {}", v.x, v.y, v.z);

    v *= 3.0;
    println!("{}, {}, {}", v.x, v.y, v.z);

    let u = Vector3::new(0., -1., 0.);
    println!("{} dot product", v.dot_product(&u));
}

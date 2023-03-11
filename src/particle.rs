use crate::vec3::Vector3;
use crate::vec3::Real;

pub struct Particle {
    position : Vector3,
    velocity : Vector3,
    pub acceleration : Vector3,
    dump : Real,
    inverse_mass : Real,
}
impl Particle {
    pub fn new(position: Vector3,
               velocity: Vector3,
               acceleration: Vector3,
               mass: Real) -> Result<Particle, &'static str> {
        let inverse_mass = if 0.0 == mass {
            return Err("Mass can't be zero.") 
        } else {
            1.0 / mass
        };

        Ok(Particle { position, velocity, acceleration, dump: 0.9, inverse_mass })
    }
    
    pub fn new_immovable(position: Vector3,
                         velocity: Vector3,
                         acceleration: Vector3) -> Particle {
        Particle { position, velocity, acceleration, dump: 0.9 ,inverse_mass: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_particle_with_zero_mass_test() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(0.0, 0.0, 0.0);
        let mass: Real = 0.0;

        let res = Particle::new(position, velocity, acceleration, mass);
        assert!(res.is_err());
        assert_eq!(res.err(), Some("Mass can't be zero."));
    }

    #[test]
    fn new_particle_with_inf_mass_test() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(0.0, 0.0, 0.0);
        let mass: Real = Real::INFINITY;

        let res = Particle::new(position, velocity, acceleration, mass);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().inverse_mass, 0.0);
    }

    #[test]
    fn new_immovable_particle_test() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(0.0, 0.0, 0.0);

        let res = Particle::new_immovable(position, velocity, acceleration);
        assert_eq!(res.inverse_mass, 0.0);
    }
}

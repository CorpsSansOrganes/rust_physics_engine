use crate::vec3::Vector3;
use crate::vec3::Real;

pub struct Particle {
    position : Vector3,
    velocity : Vector3,
    pub acceleration : Vector3,
    forces : Vector3,
    damp : Real,
    inverse_mass : Real,
}
impl Particle {
    pub fn new(position: Vector3,
               velocity: Vector3,
               acceleration: Vector3,
               mass: Real) -> Result<Particle, &'static str> {
        let inverse_mass = if 0.0 >= mass {
            return Err("Illegal mass.") 
        } else {
            1.0 / mass
        };
        let forces = Vector3::new(0.0, 0.0, 0.0);

        Ok(Particle { position, velocity, acceleration, forces, damp: 0.9, inverse_mass })
    }
    
    pub fn new_immovable(position: Vector3,
                         velocity: Vector3,
                         acceleration: Vector3) -> Particle {
        let forces = Vector3::new(0.0, 0.0, 0.0);
        Particle { position, velocity, acceleration, forces, damp: 0.9 ,inverse_mass: 0.0 }
    }

    pub fn integrate(&mut self, duration: Real) {
        /*
         * Integrates the partical's position after a given duration,
         * using Newton-Euler integration method.
         */

        debug_assert!(duration > 0.0, "Illegal duration"); 

        // Update position
        self.position += (self.velocity + self.acceleration * duration * 0.5) * duration;

        // Work out the acceleration from forces accumlated.
        let res_acc = self.acceleration + self.forces * self.inverse_mass;

        // Update velocity
        self.velocity += res_acc * duration;

        // Impose drag
        self.velocity *= self.damp.powf(duration);
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
        assert_eq!(res.err(), Some("Illegal mass."));
    }

    #[test]
    fn new_particle_with_negative_mass_test() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(0.0, 0.0, 0.0);
        let mass: Real = -1.0;

        let res = Particle::new(position, velocity, acceleration, mass);
        assert!(res.is_err());
        assert_eq!(res.err(), Some("Illegal mass."));
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

    #[should_panic]
    #[test]
    fn integrate_with_zero_duration() {
        let position = Vector3::new(1.3, 2.1, -0.3);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(1.243, -3.0, 0.3333);
        let mass: Real = 3.0;

        let mut particle = Particle::new(position, velocity, acceleration, mass).unwrap();
        particle.integrate(0.0);
    }

    #[should_panic]
    #[test]
    fn integrate_with_negative_duration() {
        let position = Vector3::new(1.3, 2.1, -0.3);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(1.243, -3.0, 0.3333);
        let mass: Real = 3.0;

        let mut particle = Particle::new(position, velocity, acceleration, mass).unwrap();
        particle.integrate(-1.0);
    }

    #[ignore]
    #[test]
    fn integrate_stress_test() {
        let position = Vector3::new(1.3, 2.1, -0.3);
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let acceleration = Vector3::new(1.243, -3.0, 0.3333);
        let mass: Real = 3.0;

        let mut particle = Particle::new(position, velocity, acceleration, mass).unwrap();

        let time = 1.0 / 60.0;
        for _i in 0..100000000 {
            particle.integrate(time);
        }
    }
}

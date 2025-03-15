use uom::si::{
    f64::{Length, Mass, Momentum, Velocity},
    mass::gram,
};

use crate::{
    GALAXY_VELOCITY, ORBIT_VELOCITY, PLANCK_CONSTANT, SOLAR_SYSTEM_VELOCITY, SPIN_VELOCITY,
    SUN_VELOCITY,
};

pub fn wvl(kg: Mass) {
    let total_velocity: Velocity =
        SPIN_VELOCITY + ORBIT_VELOCITY + SUN_VELOCITY + SOLAR_SYSTEM_VELOCITY + GALAXY_VELOCITY; // ???
    let mass: Mass = Mass::new::<gram>(kg.get::<gram>());
    let velocity: Velocity = total_velocity;
    let momentum: Momentum = mass * velocity;
    let wavelength = calc_wavelength(momentum);
    println!("{:?}", wavelength);
}

pub fn calc_wavelength(momentum: Momentum) -> Length {
    PLANCK_CONSTANT / momentum
}

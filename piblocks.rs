use uom::si::{
    f64::{Mass, Velocity},
    mass::kilogram,
    velocity::meter_per_second,
};

use crate::energy;

pub fn calc(block_mass: Mass, block_velocity: Velocity) -> u32 {
    let collisions = 0;
    let m1 = Mass::new::<kilogram>(1f64);
    let v1 = Velocity::new::<meter_per_second>(1f64);
    let m2 = block_mass;
    let v2 = block_velocity;

    let energy1 = energy(m1, v1);
    let energy2 = energy(m2, v2);
    let total_energy = energy1 + energy2;

    let momentum1 = m1 * v1;
    let momentum2 = m2 * v2;
    let total_momentum = momentum1 + momentum2;

    print!("{:?} {:?}", total_energy, total_momentum);

    collisions
}

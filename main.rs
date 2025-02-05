use uom::si::{
    energy::joule,
    f64::{Acceleration, Energy, Length, Mass, Momentum, Time, Velocity},
    length::meter,
    mass::{gram, kilogram},
    time::second,
};

extern crate uom;

fn main() {
    let mass_kg: Mass = Mass::new::<kilogram>(120.0);
    let length = Length::new::<meter>(5.0);
    let time = Time::new::<second>(1.0);
    let velocity: Velocity = length / time;
    let acceleration = calc_acceleration(velocity, time);
    let mass: Mass = Mass::new::<gram>(mass_kg.get::<gram>());
    let momentum: Momentum = mass * velocity;
    let wavelength = calc_wavelength(momentum);
    //let error = length + time; // error[E0308]: mismatched types
    println!("{:?}, {:?}, {:?}", mass_kg, acceleration, wavelength);
}

fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
    velocity / time
}

fn calc_wavelength(momentum: Momentum) -> Length {
    let planck_constant = Energy::new::<joule>(6.626_070_15e-34) * Time::new::<second>(1.0);
    planck_constant / momentum
}

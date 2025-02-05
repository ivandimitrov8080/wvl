use uom::si::{
    energy::joule,
    f64::{Energy, Length, Mass, Momentum, Time, Velocity},
    mass::{gram, kilogram},
};

extern crate uom;

static SECOND: Time = seconds(1.0);
static SPIN_VELOCITY: Velocity = meter_second(463.88889); // 1670 km/h
static ORBIT_VELOCITY: Velocity = meter_second(30000.0); // 108 000 km/h
static SUN_VELOCITY: Velocity = meter_second(19444.44444); // 70 000 km/h
static SOLAR_SYSTEM_VELOCITY: Velocity = meter_second(220000.0); // 792 000 km/h
static GALAXY_VELOCITY: Velocity = meter_second(694444.44444); // 2 500 000 km/h

const fn seconds(value: f64) -> Time {
    Time {
        dimension: std::marker::PhantomData,
        units: std::marker::PhantomData,
        value,
    }
}

const fn meter_second(value: f64) -> Velocity {
    Velocity {
        dimension: std::marker::PhantomData,
        units: std::marker::PhantomData,
        value,
    }
}

fn main() {
    let total_velocity: Velocity =
        SPIN_VELOCITY + ORBIT_VELOCITY + SUN_VELOCITY + SOLAR_SYSTEM_VELOCITY + GALAXY_VELOCITY; // ???
    let mass_kg: Mass = Mass::new::<kilogram>(120.0);
    let mass: Mass = Mass::new::<gram>(mass_kg.get::<gram>());
    let velocity: Velocity = total_velocity;
    let momentum: Momentum = mass * velocity;
    let wavelength = calc_wavelength(momentum);
    println!("{:?}", wavelength);
}

fn calc_wavelength(momentum: Momentum) -> Length {
    let planck_constant = Energy::new::<joule>(6.626_070_15e-34) * SECOND;
    planck_constant / momentum
}

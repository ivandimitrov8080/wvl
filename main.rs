mod triangles;
mod twgpu;
use std::{env, marker::PhantomData};

use triangles::triangles;
use twgpu::run;
use uom::si::{
    f64::{Action, Length, Mass, Momentum, Velocity},
    mass::{gram, kilogram},
};

extern crate uom;

static PLANCK_CONSTANT: Action = joules_second(6.626_070_15e-34);
static SPIN_VELOCITY: Velocity = meter_second(463.88889); // 1670 km/h
static ORBIT_VELOCITY: Velocity = meter_second(30000.0); // 108 000 km/h
static SUN_VELOCITY: Velocity = meter_second(19444.44444); // 70 000 km/h
static SOLAR_SYSTEM_VELOCITY: Velocity = meter_second(220000.0); // 792 000 km/h
static GALAXY_VELOCITY: Velocity = meter_second(694444.44444); // 2 500 000 km/h

const fn meter_second(value: f64) -> Velocity {
    Velocity {
        dimension: PhantomData,
        units: PhantomData,
        value,
    }
}
const fn joules_second(value: f64) -> Action {
    Action {
        dimension: PhantomData,
        units: PhantomData,
        value,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "wvl" => {
            let kg: f64 = args[2].parse().unwrap();
            wvl(kg);
        }
        "tri" => {
            triangles();
        }
        "twgpu" => {
            match run() {
                Ok(_) => print!("alright"),
                Err(_) => print!("OY WAT R U DOIN"),
            };
        }
        _ => panic!("OY"),
    }
}

fn wvl(kg: f64) {
    let total_velocity: Velocity =
        SPIN_VELOCITY + ORBIT_VELOCITY + SUN_VELOCITY + SOLAR_SYSTEM_VELOCITY + GALAXY_VELOCITY; // ???
    let mass_kg: Mass = Mass::new::<kilogram>(kg);
    let mass: Mass = Mass::new::<gram>(mass_kg.get::<gram>());
    let velocity: Velocity = total_velocity;
    let momentum: Momentum = mass * velocity;
    let wavelength = calc_wavelength(momentum);
    println!("{:?}", wavelength);
}

fn calc_wavelength(momentum: Momentum) -> Length {
    PLANCK_CONSTANT / momentum
}

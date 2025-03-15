mod triangles;
mod twgpu;
mod wvl;
use std::{env, marker::PhantomData};

use triangles::triangles;
use twgpu::run;
use uom::si::f64::{Action, Velocity};
use wvl::wvl;

extern crate uom;

pub static PLANCK_CONSTANT: Action = joules_second(6.626_070_15e-34);
pub static SPIN_VELOCITY: Velocity = meter_second(463.88889); // 1670 km/h
pub static ORBIT_VELOCITY: Velocity = meter_second(30000.0); // 108 000 km/h
pub static SUN_VELOCITY: Velocity = meter_second(19444.44444); // 70 000 km/h
pub static SOLAR_SYSTEM_VELOCITY: Velocity = meter_second(220000.0); // 792 000 km/h
pub static GALAXY_VELOCITY: Velocity = meter_second(694444.44444); // 2 500 000 km/h

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
        "piblocks" => {
            match run() {
                Ok(_) => print!("alright"),
                Err(_) => print!("OY WAT R U DOIN"),
            };
        }
        _ => panic!("OY"),
    }
}

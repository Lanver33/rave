use std::error::Error;

use rave_lib::{*};
use uom::si::f32::Time;
use uom::si::time::second;

fn main() -> Result<(), Box<dyn Error>> {
    let total_time : Time = Time::new::<second>(10.0);
    let samples = white_noise(total_time, 44100);
    generate_wave_file("white_noise", &samples)?;
    Ok(())
}

use std::{i16};
use hound::{self};
use uom::si::f32::Time;
use uom::si::time::{second};
use rand::{RngExt};
pub type SampleIntensity = i16;

fn normalize_volume(intensity : SampleIntensity) -> SampleIntensity
{
    intensity * 0.01 as SampleIntensity
}

pub fn white_noise(time : Time, sample_rate : i32) -> Vec<SampleIntensity> {
    let mut rng = rand::rng();
    let mut samples: Vec<SampleIntensity> = Vec::new();
    let total_samples = (time.get::<second>() * sample_rate as f32) as i64;
    for _ in 0..total_samples {
        samples.push(rng.random_range(SampleIntensity::MIN..normalize_volume(SampleIntensity::MAX)));
    }
    samples
}
pub fn generate_wave_file(file_name : &str, samples : &Vec<SampleIntensity>) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(format!("{file_name}.wav"), spec)?;
    for sample in samples {
        writer.write_sample(*sample)?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_noise() {
        let white_noise_samples = white_noise(
            Time::new::<second>(1.0), 44100);
        assert!(white_noise_samples.len() == 44100);
        let n = white_noise_samples.len() as f64;

        let average_value : f64 = white_noise_samples.iter().map(|&x|x as f64).sum::<f64>() / n;
        let mut auto_covariance_matrix = [[0; 44100]; 44100];
    }
}

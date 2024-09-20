use std::ffi::CString;
use std::ops::RangeInclusive;

use alsa::pcm::{Access, Format, HwParams, State, PCM};
use alsa::{Direction, ValueOr};
use anyhow::Context;
use clap::Parser;

/// Simple program to play an audio note by frequency
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Frequency (HZ) of the note to play
    #[clap(short, long, value_parser = frequency_in_range, default_value_t = 261.626)]
    frequency: f32,

    /// Amplitude (volume) of the sample, in the range [0.0, 1.0]
    #[clap(short, long, value_parser = amplitude_in_range, default_value_t = 0.5)]
    amplitude: f32,

    /// Duration of the sample, in the range [0.0, 5.0]
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    duration: f32,
}

const SAMPLE_RATE: u32 = 44100;

fn pcm_open_default() -> Result<PCM, anyhow::Error> {
    let pcm = PCM::open(
        &CString::new("default").unwrap(),
        Direction::Playback,
        false,
    )
    .context("PCM::open() failed")?;

    {
        let hwp = HwParams::any(&pcm)?;
        hwp.set_channels(1)?;
        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest)?;
        hwp.set_format(Format::s16())?;
        hwp.set_access(Access::RWInterleaved)?;
        pcm.hw_params(&hwp)?;

        let hwp = pcm.hw_params_current()?;
        let swp = pcm.sw_params_current()?;
        swp.set_start_threshold(hwp.get_buffer_size()?)?;
        pcm.sw_params(&swp)?;
    }

    Ok(pcm)
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    println!("frequency: {}", args.frequency);
    println!("amplitude: {}", args.amplitude);
    println!("duration: {}", args.duration);

    let pcm = pcm_open_default()?;
    println!(
        "PCM status: {:?}, {:?}",
        pcm.state(),
        pcm.hw_params_current()?
    );

    // create samples in a i16 buffer
    let buf_sz: usize = ((SAMPLE_RATE as f32) / args.frequency) as usize;

    let mut buf = Vec::with_capacity(buf_sz);
    for i in 0..buf_sz {
        let mut val: f32 =
            (i as f32 * std::f32::consts::TAU / ((SAMPLE_RATE as f32) / args.frequency)).sin();
        val *= args.amplitude;
        val *= i16::MAX as f32;
        buf.push(val as i16);
    }

    // play the samples
    let io = pcm.io_i16()?;
    for _ in 0..(args.duration * ((SAMPLE_RATE as usize / buf_sz) as f32)) as usize {
        let samples_out = io.writei(&buf[..])?;
        if samples_out != buf_sz {
            return Err(anyhow::anyhow!(
                "Output samples {} not equal to buffer size {}",
                samples_out,
                buf_sz
            ));
        }
    }

    if pcm.state() != State::Running {
        pcm.start()?;
    }

    pcm.drain().context("PCM failed to drain()")?;

    Ok(())
}

fn f32_arg_in_range(s: &str, range: &RangeInclusive<f32>) -> Result<f32, String> {
    let val: f32 = s
        .parse()
        .map_err(|_| format!("`{}` is not a floating point number", s))?;
    if range.contains(&val) {
        Ok(val)
    } else {
        Err(format!(
            "Value must be in range [{}, {}]",
            range.start(),
            range.end()
        ))
    }
}

const FREQUENCY_RANGE: RangeInclusive<f32> = 0.0..=f32::MAX;

fn frequency_in_range(s: &str) -> Result<f32, String> {
    f32_arg_in_range(s, &FREQUENCY_RANGE)
}

const AMPLITUDE_RANGE: RangeInclusive<f32> = 0.0..=1.0;

fn amplitude_in_range(s: &str) -> Result<f32, String> {
    f32_arg_in_range(s, &AMPLITUDE_RANGE)
}

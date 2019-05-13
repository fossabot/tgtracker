use rb::{RbConsumer, RbProducer, SpscRb, RB};
use rsoundio;
use std::f32::consts::PI as PI32;
use std::thread;

// use std::time::Duration;
use yansi::Paint;
/// Buffer size
const BUF_SIZE: usize = 2048; // TODO: Make this configurable

/// Initialize cross-platform audio using libsoundio
pub fn init() {
    // ringbuffer
    let rb = SpscRb::new(BUF_SIZE);
    let (producer, consumer) = (rb.producer(), rb.consumer());

    // create an audio context
    let mut sio = rsoundio::SoundIo::default();
    sio.set_name("tgtracker-audio-context").unwrap();

    // connect to the default audio backend
    sio.connect().unwrap();
    let backend = sio.current_backend().unwrap();
    println!("Connected to backend: {}", Paint::yellow(backend));
    sio.flush_events();

    // get default output device
    let dev = sio.default_output_device().unwrap();
    assert!(dev.probe_error().is_none());
    println!("Using output device: {}", dev);

    // create output stream
    let mut out = dev.create_outstream().unwrap();
    assert!(out.set_name("tgtracker-audio-out").is_ok());
    out.set_format(rsoundio::SioFormat::Float32LE).unwrap();
    println!("Output format: {}", Paint::yellow(out.format().unwrap()));
    thread::spawn(move || {
        const LEN: usize = BUF_SIZE / 16;
        let mut pos = 0;
        loop {
            const F: f32 = 440.0;
            const W: f32 = 2.0 * F * PI32 / 48_000.0;
            const A: f32 = 0.6;
            const CYCLE: usize = (48_000f32 / F) as usize;
            let samples: Vec<f32> = (0..LEN).map(|i| (W * (i + pos) as f32).sin() * A).collect();
            producer.write_blocking(&samples).unwrap();
            pos = (pos + LEN) % CYCLE;
        }
    });

    // register callbacks
    out.register_write_callback(
        |out: rsoundio::OutStream, min_frame_count: u32, max_frame_count: u32| {
            let mut frames_left = max_frame_count as usize;
            let mut buf = vec![0.0f32; BUF_SIZE];
            while frames_left > 0 {
                let len = ::std::cmp::min(BUF_SIZE, frames_left);
                consumer.read_blocking(&mut buf[..len]);
                let left = buf[..len].to_vec();
                let right = left.clone();
                let frames = vec![left, right];
                let cnt = out.write_stream_f32(min_frame_count, &frames).unwrap() as usize;
                frames_left -= cnt;
            }
        },
    );
    out.register_underflow_callback(|out: rsoundio::OutStream| {
        println!(
            "Underflow in {} occured!",
            Paint::yellow(out.name().unwrap())
        )
    });
    out.register_error_callback(|out: rsoundio::OutStream, err: rsoundio::SioError| {
        println!(
            "{} error: {}",
            Paint::yellow(out.name().unwrap()),
            Paint::red(err)
        )
    });

    // open output stream
    out.open().unwrap();
    let sample_rate = out.sample_rate();
    println!("Sample rate: {}Hz", Paint::yellow(sample_rate));
    // lossless cast from sample_rate to f64
    out.set_latency(f64::from(sample_rate) / BUF_SIZE as f64);
    match out.latency() {
        Ok(latency) => println!("SW latency: {:4.2}ms", Paint::yellow(latency * 1000.0)),
        Err(err) => println!("err: {}", Paint::red(err)),
    }
    let layout = out.layout();
    println!("Output channel layout: {}", Paint::yellow(layout));
    // start audio output (now the `write_callback` will be called periodically)
    // out.start().unwrap();
    // thread::sleep(Duration::new(3, 0));
    // println!("Pause for 1s");
    // out.pause();
    // thread::sleep(Duration::new(1, 0));
    // println!("Unpausing");
    // out.unpause();
    // thread::sleep(Duration::new(3, 0));
}

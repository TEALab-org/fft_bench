mod build_info;
mod plan_type;

use clap::Parser;
use fftw::array::*;
use fftw::plan::*;
use fftw::types::c64;
use plan_type::*;
use rand::distr::Uniform;
use rand::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Instant;

use serde::Serialize;

use crate::build_info::{git_describe, git_hash};

/// fft_bench is a tool for timing different FFTw3 + Rust + Rayon plan variations.
#[derive(Parser, Debug, Serialize)]
struct Args {
    /// Size of fft plan
    #[arg(long)]
    plan_size: usize,

    /// The number of threads to use
    #[arg(long)]
    threads: usize,

    /// The FFTW plan type
    #[arg(long)]
    plan_type: PlanType,

    /// Number of repetitions
    #[arg(long)]
    test_count: usize,

    /// Timings file path
    #[arg(long)]
    output: PathBuf,

    /// FFTW3 Wisdom
    #[arg(long)]
    wisdom_path: PathBuf,

    /// Print build info and exit
    #[arg(long)]
    build_info: bool,
}

#[derive(Serialize)]
struct ResultFile {
    threads: usize,
    plan_type: PlanType,
    plan_size: usize,
    git_describe: String,
    git_hash: String,
    timings: Vec<u128>,
}

impl ResultFile {
    pub fn from_args(args: &Args) -> Self {
        Self {
            threads: args.threads,
            plan_type: args.plan_type,
            plan_size: args.plan_size,
            git_describe: git_describe().to_string(),
            git_hash: git_hash().to_string(),
            timings: Vec::with_capacity(args.test_count),
        }
    }
}

/// Utility function to ensure output directories exist when needed
pub fn ensure_dir_exists<P: AsRef<std::path::Path>>(path: &P) {
    // Check if it exists
    let p = path.as_ref();
    if p.exists() {
        if p.is_dir() {
            println!("Exists: {p:?}");
        } else {
            panic!("ERROR: not a directory {p:?}");
        }
    } else {
        println!("Creating: {p:?}");
        if let Err(e) = std::fs::create_dir_all(p) {
            panic!("ERROR: failed to create {p:?}, {e}");
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("FFT_BENCH: Start");
    println!("{}", serde_json::to_string_pretty(&args).unwrap());

    if args.build_info {
        build_info::print_report("fft_bench");
        std::process::exit(0);
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();
    fftw::threading::init_threads_f64().unwrap();

    // This is setting a default value,
    // Not strictly necessary,
    // i.e. we should set this explicitly whenever planning
    fftw::threading::plan_with_nthreads_f64(args.threads);

    let parent_path = args.wisdom_path.parent().unwrap();
    ensure_dir_exists(&parent_path);
    if args.wisdom_path.exists() {
        println!("FFT_BENCH: Loading Wisdom File");
        fftw::wisdom::import_wisdom_file_f64(&args.wisdom_path).unwrap();
    } else {
        println!("FFT_BENCH: Wisdom file doesn't exist");
    }

    let mut results = ResultFile::from_args(&args);

    // Create Buffers
    let mut real_buffer: AlignedVec<f64> = AlignedVec::new(args.plan_size);
    let mut complex_buffer: AlignedVec<c64> = AlignedVec::new(args.plan_size / 2 + 1);

    println!("FFT_BENCH: Create Initial Conditions");
    let mut rng = rand::rng();
    let dist = Uniform::new(-1.0, 1.0).unwrap();
    let mut sum = 0.0;
    for i in 0..args.plan_size {
        let value = rng.sample(dist);
        sum += value;
        real_buffer[i] = value;
    }
    println!("FFT_BENCH: IC Sum {}", sum);

    println!("FFT_BENCH: Create FFT Plans");
    fftw::threading::plan_with_nthreads_f64(args.threads);
    let forward_plan =
        fftw::plan::R2CPlan64::aligned(&[args.plan_size], args.plan_type.to_fftw3_flag()).unwrap();
    let backward_plan =
        fftw::plan::C2RPlan64::aligned(&[args.plan_size], args.plan_type.to_fftw3_flag()).unwrap();

    for test in 0..args.test_count {
        let start_time = Instant::now();

        forward_plan
            .r2c(&mut real_buffer, &mut complex_buffer)
            .unwrap();
        backward_plan
            .c2r(&mut complex_buffer, &mut real_buffer)
            .unwrap();

        let end_time = Instant::now();
        let duration = (end_time - start_time).as_nanos();
        results.timings.push(duration);
        println!("FFT_BENCH: test {}, duration {} ns", test, duration);
    }

    // New sum?
    let final_sum: f64 = real_buffer.iter().sum();
    println!("FFT_BENCH: final sum {}", final_sum);

    // Write output
    println!("FFT_BENCH: Writing Output");
    let output_file = File::create(args.output).unwrap();
    let mut output_writer = BufWriter::new(output_file);
    serde_json::to_writer_pretty(&mut output_writer, &results).unwrap();

    println!("FFT_BENCH: Writing wisdom");
    fftw::wisdom::export_wisdom_file_f64(&args.wisdom_path).unwrap();

    println!("FFT_BNECH: End");
}

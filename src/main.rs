mod cpu_measure;
mod process_info;
mod configuration;
mod memory_measure;
mod process_info_tests;
mod consts;

use clap::Parser;
use crate::configuration::config::Config;
use crate::process_info::ProcessInfo;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "process-watcher-rs", version)]
#[command(about = "Tool to watch a given process", long_about = None)]
struct Args {
    /// PID of the process to be watched
    #[arg(short, long, default_value_t = -1)]
    pid: i32,

    /// Path to JSON configuration file
    #[arg(short, long, default_value = "")]
    configuration: String,

    /// Debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Output path
    #[arg(short, long, default_value = "./")]
    output: String,
}

fn print_args(args: &Args) {
    println!("PID {}", args.pid);
    println!("Configuration file {}", args.configuration);
    println!("Debug mode {}", args.debug);
    println!("Output {}", args.output);
}

fn main() {

    let args = Args::parse();
    println!("===== INPUT ARGS =====");
    print_args(&args);

    let c = if args.configuration == ""  { Config::default() } else { Config::from_json(args.configuration).unwrap() };

    println!("===== CONFIGURATION =====");
    c.print();


    if args.pid != -1 { // PID takes precedence over command in configuration file
        let mut process_info = ProcessInfo::new(args.pid, args.debug, args.output, c);
        let handler_process_info = std::thread::spawn(move|| {
            process_info.run();
        });

        handler_process_info.join().unwrap();
    }
}

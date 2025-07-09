mod process_info;
mod configuration;
mod measures;
mod process_info_tests;
mod common;

use clap::Parser;
use crate::configuration::config::Config;
use crate::process_info::ProcessInfo;
use std::process::Command;


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

fn main() {

    let args = Args::parse();
    println!("{:#?}", args);

    let c = if args.configuration == ""  { Config::default() } else { Config::from_json(args.configuration).unwrap() };

    println!("{:#?}", c);


    if args.pid != -1 { // PID takes precedence over command in configuration file
        let mut process_info = ProcessInfo::new(args.pid, args.debug, args.output, c);
        let handler_process_info = std::thread::spawn(move|| {
            process_info.run();
        });

        handler_process_info.join().unwrap();
    } else {
        match c.command.clone() {
            Some(command) => {
                let command_parts = command.split_whitespace().collect::<Vec<&str>>();
                let trigger_command = Command::new(command_parts[0])
                    .args(command_parts[1..].iter()).spawn().unwrap();

                let mut process_info = ProcessInfo::new(trigger_command.id() as i32, args.debug, args.output, c);
                let handler_process_info = std::thread::spawn(move|| {
                    process_info.run();
                });

                handler_process_info.join().unwrap();
            }
            None => {println!("No command set in Configuration. Bye!");}
        }

    }
}

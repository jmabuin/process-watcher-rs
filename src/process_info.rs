use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use procfs::process::Process;
use crate::configuration::config::Config;
use crate::cpu_measure::CpuMeasure;
use crate::memory_measure::MemoryMeasure;
use crate::common::consts::constants;

pub struct ProcessInfo {
    pub pid: i32,
    pub is_debug_mode: bool,
    pub output_folder: String,
    pub config: Config,
    pub cpu_measures: Vec<CpuMeasure>,
    pub memory_measures: Vec<MemoryMeasure>,
}

impl ProcessInfo {
    pub fn new(pid: i32, is_debug_mode: bool, output_folder: String, config: Config) -> Self {
        let cpu_measures: Vec<CpuMeasure> = Vec::new();
        let memory_measures: Vec<MemoryMeasure> = Vec::new();
        ProcessInfo {pid, is_debug_mode, output_folder, config, cpu_measures, memory_measures}
    }

    pub fn run(&mut self) {
        let p = Process::new(self.pid);
        let sleep_duration = Duration::from_secs_f64(self.config.measure_interval);
        let tps = procfs::ticks_per_second();
        let page_size = procfs::page_size();

        match p {
            Ok(process) => {
                let mut measure_time: f64 = 0.0;
                let mut old_cpu_time: u64 = 0;

                while process.is_alive() {

                    let stat = process.stat().unwrap();

                    // CPU
                    if old_cpu_time == 0 {
                        old_cpu_time = (stat.utime + stat.stime) / tps;
                    }
                    let new_cpu_time = (stat.utime + stat.stime) / tps;
                    let cpu_percentage = (new_cpu_time - old_cpu_time) as f64 * 100.0 / measure_time;
                    if !cpu_percentage.is_nan() { // First result will always be NaN
                        let new_cpu_measure = CpuMeasure::new(measure_time, cpu_percentage);
                        self.cpu_measures.push(new_cpu_measure);
                    }

                    // Memory
                    let memory = stat.rss * page_size; // Bytes
                    let new_memory_measure = MemoryMeasure::new(measure_time, memory);
                    self.memory_measures.push(new_memory_measure);
                    if self.is_debug_mode && !cpu_percentage.is_nan() {
                        println!("[{}] Time: {} CPU: {}%, Mem: {}", self.pid, measure_time, cpu_percentage, memory);
                    }
                    sleep(sleep_duration);
                    measure_time += self.config.measure_interval;
                }

                let write_result = self.save_results();

                match write_result {
                    Ok(()) => {}
                    Err(e) => {
                        panic!("Could not write output files! {}", e);
                    }
                }
            }
            Err(e) => {
                panic!("Process with PID {} can not be found/accessed because of {}", self.pid, e)
            }
        }
    }

    pub fn save_results(&self) -> std::io::Result<()> {
        let cpu_file_path = Path::new(self.output_folder.as_str()).join(self.pid.to_string() + constants::CPU_MEASURES_SUFFIX);
        let mem_file_path = Path::new(self.output_folder.as_str()).join(self.pid.to_string() + constants::MEMORY_MEASURES_SUFFIX);
        let cpu_file_name = cpu_file_path.to_str().unwrap();
        let mem_file_name = mem_file_path.to_str().unwrap();

        let mut cpu_file = File::create(cpu_file_name)?;

        writeln!(&mut cpu_file, "Time;Percentage")?;
        for cpu_measure in self.cpu_measures.iter() {
            writeln!(&mut cpu_file, "{};{}", cpu_measure.time_seconds, cpu_measure.percentage)?;
        }

        let mut mem_file = File::create(mem_file_name)?;

        writeln!(&mut mem_file, "Time;Memory")?;
        for mem_measure in self.memory_measures.iter() {
            writeln!(&mut mem_file, "{};{}", mem_measure.time_seconds, mem_measure.quantity)?;
        }

        Ok(())
    }
}
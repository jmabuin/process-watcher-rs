use std::thread::sleep;
use std::time::Duration;
use procfs::process::Process;
use crate::config::Config;
use crate::cpu_measure::CpuMeasure;
use crate::memory_measure::MemoryMeasure;

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
        let sleep_duration = Duration::new(self.config.measure_interval, 0);
        let tps = procfs::ticks_per_second();
        let page_size = procfs::page_size();

        match p {
            Ok(process) => {
                let mut measure_time: u64 = 0;
                let mut old_cpu_time: u64 = 0;

                while process.is_alive() {

                    let stat = process.stat().unwrap();

                    // CPU
                    if old_cpu_time == 0 {
                        old_cpu_time = (stat.utime + stat.stime) / tps;
                    }
                    let new_cpu_time = (stat.utime + stat.stime) / tps;
                    let cpu_percentage = (new_cpu_time - old_cpu_time) as f64 * 100.0 / measure_time as f64;
                    let new_cpu_measure = CpuMeasure::new(measure_time, cpu_percentage);
                    self.cpu_measures.push(new_cpu_measure);

                    // Memory
                    let memory = stat.rss * page_size; // Bytes
                    let new_memory_measure = MemoryMeasure::new(measure_time, memory);
                    self.memory_measures.push(new_memory_measure);
                    if self.is_debug_mode {
                        println!("[{}] Time: {} CPU: {}%, Mem: {}", self.pid, measure_time, cpu_percentage, memory);
                    }
                    sleep(sleep_duration);
                    measure_time += self.config.measure_interval;
                }
            }
            Err(e) => {
                panic!("Process with PID {} can not be found/accessed because of {}", self.pid, e)
            }
        }
    }

    pub fn save_results(&self) {

    }
}
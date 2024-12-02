
#[cfg(test)]
mod process_info_tests {
    use crate::configuration::config::Config;
    use crate::configuration::config_cpu::ConfigCpu;
    use crate::configuration::config_memory::ConfigMemory;
    use crate::measures::cpu_measure::CpuMeasure;
    use crate::process_info::ProcessInfo;
// Note this useful idiom: importing names from outer (for mod tests) scope.
//use super::*;

    #[test]
    fn test_create_process_info() {
        let test_config = String::from("configs/example.json");
        let config_result = Config::from_json(test_config);

        let p = ProcessInfo::new(12, false, String::from("/tmp/output"), config_result.unwrap());

        assert_eq!(p.is_debug_mode, false);
        assert_eq!(p.pid, 12);
        println!("Process PID is {}", p.pid);
    }

}
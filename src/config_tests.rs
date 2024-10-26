#[cfg(test)]
mod config_tests {
    use crate::config::Config;
    use crate::config_cpu::ConfigCpu;
    use crate::config_memory::ConfigMemory;
    use crate::cpu_measure::CpuMeasure;
// Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    #[test]
    fn test_read_config_from_json() {
        let test_config = String::from("configs/example.json");

        let config_result = Config::from_json(test_config);

        match config_result {
            Ok(c) => {
                assert_eq!(c.command, String::from("/opt/google/chrome/chrome"));
                assert_eq!(c.cpu.measure_cpu, true);
                assert_eq!(c.measure_interval, 5.0);
                assert_eq!(c.memory.measure_memory, true);
                assert_eq!(c.memory.memory_units, String::from("MB"));
            }
            Err(err) => panic!("{}", err)
        };
    }

    #[test]
    fn test_create_config() {
        let cpu_measure_config = ConfigCpu {
            measure_cpu: true
        };

        let memory_measure_config = ConfigMemory {
            measure_memory: false,
            memory_units: String::from("KB"),
        };

        let c = Config {
            measure_interval: 2.0,
            cpu: cpu_measure_config,
            memory: memory_measure_config,
            command: String::from("/usr/bin/ls"),
        };

        assert_eq!(c.command, String::from("/usr/bin/ls"));
        assert_eq!(c.cpu.measure_cpu, true);
        assert_eq!(c.measure_interval, 2.0);
        assert_eq!(c.memory.measure_memory, false);
        assert_eq!(c.memory.memory_units, String::from("KB"));
    }

    #[test]
    fn test_create_cpu_measure() {
        let cpu_measure = CpuMeasure::new(324.0, 34.67);
        assert_eq!(cpu_measure.time_seconds, 324.0);
        assert_eq!(cpu_measure.percentage, 34.67);
    }
}
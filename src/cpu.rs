use num_cpus;
use rand::{self, Rng};
use std::thread;
use std::time::Duration;
pub struct CpuLoad {
    // second
    delay_start: (Duration, Duration),
    // MB
    cpu_range: (usize, usize),
    // second
    run_range: (Duration, Duration),
}

pub struct CpuLoadBuilder {
    // second
    delay_start: (Duration, Duration),
    // MB
    cpu_range: (usize, usize),
    // second
    run_range: (Duration, Duration),
}

impl CpuLoad {
    pub fn new() -> CpuLoadBuilder {
        CpuLoadBuilder {
            delay_start: (Duration::from_secs(0), Duration::from_secs(0)),
            cpu_range: (30, 70),
            run_range: (Duration::from_secs(60), Duration::from_secs(120)),
        }
    }
    pub fn run(self) {
        let mut rng = rand::thread_rng();
        let ds = rng.gen_range(self.delay_start.0..self.delay_start.1);
        thread::sleep(ds);
        let util = (rng.gen_range(self.cpu_range.0..self.cpu_range.1) as f64) / 100.0;
        let t_t = rng.gen_range(self.run_range.0..self.run_range.1);

        let num = num_cpus::get();
        let th_num = (util * num as f64) as usize;
        println!("cpu util:{}", util);

        for _ in 0..th_num {
            thread::spawn(|| {
                loop {
                    // for i in 0..util {}
                    // thread::sleep(t_t);
                    // if cnt == t_t / util {
                    //     break;
                    // }
                }
            });
        }

        thread::sleep(t_t)
    }
}

impl CpuLoadBuilder {
    pub fn delay_start(&mut self, dt: (u64, u64)) -> &mut Self {
        self.delay_start = (Duration::from_secs(dt.0), Duration::from_secs(dt.1));
        self
    }
    pub fn cpu_range(&mut self, cpu: (usize, usize)) -> &mut Self {
        self.cpu_range = cpu;
        self
    }
    pub fn run_range(&mut self, run: (u64, u64)) -> &mut Self {
        self.run_range = (Duration::from_secs(run.0), Duration::from_secs(run.1));
        self
    }
    pub fn build(&self) -> CpuLoad {
        CpuLoad {
            delay_start: self.delay_start,
            cpu_range: self.cpu_range,
            // second
            run_range: self.run_range,
        }
    }
}

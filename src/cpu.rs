use rand::{self, Rng};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct CpuLoad {
    // second
    delay_start: (Duration, Duration),
    // MB
    cpu_range: (usize, usize),
    // second
    run_range: (u64, u64),
}

pub struct CpuLoadBuilder {
    // second
    delay_start: (Duration, Duration),
    // MB
    cpu_range: (usize, usize),
    // second
    run_range: (u64, u64),
}

impl CpuLoad {
    pub fn new() -> CpuLoadBuilder {
        CpuLoadBuilder {
            delay_start: (Duration::from_secs(0), Duration::from_secs(1)),
            cpu_range: (30, 70),
            run_range: (0, 1),
        }
    }
    pub fn run(self) {
        let mut rng = rand::thread_rng();
        let ds = rng.gen_range(self.delay_start.0..self.delay_start.1);
        thread::sleep(ds);
        let util = rng.gen_range(self.cpu_range.0..self.cpu_range.1);
        let t_t = rng.gen_range(self.run_range.0..self.run_range.1);

        let num = num_cpus::get();
        // let th_num = (util * num as f64) as usize;
        println!("cpu core:{}", num);
        println!("cpu util:{}", util);
        // println!("auctal util:{}", th_num);

        // for _ in 0..th_num {
        //     thread::spawn(|| {
        //         loop {
        //             // for i in 0..util {}
        //             // thread::sleep(t_t);
        //             // if cnt == t_t / util {
        //             //     break;
        //             // }
        //         }
        //     });
        // }

        Command::new("stress-ng")
            .arg("-c")
            .arg("0")
            .arg("-l")
            .arg(util.to_string())
            .arg("--timeout")
            .arg(t_t.to_string())
            .status()
            .unwrap();

        // thread::sleep(Duration::from_secs(t_t))
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
        self.run_range = run;
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

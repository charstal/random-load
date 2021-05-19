use rand::{self, Rng};
use std::fmt;
use std::mem::size_of;
use std::thread;
use std::{time::Duration, vec::Vec};

pub struct MemoryLoad {
    // second
    delay_start: (Duration, Duration),
    // MB
    mem_range: (usize, usize),
    // second
    run_range: (Duration, Duration),
}

pub struct MemoryLoadBuilder {
    // second
    delay_start: (Duration, Duration),
    // MB
    mem_range: (usize, usize),
    // second
    run_range: (Duration, Duration),
}

pub struct MemoryLoadUnit {
    pub a: i64,
}

const RADIX: usize = 1024;

impl MemoryLoadUnit {
    fn new(a: i64) -> Self {
        MemoryLoadUnit { a }
    }
    fn unit_count(cap: usize) -> usize {
        cap * RADIX * RADIX / size_of::<Self>()
    }

    fn unit_count_range(cap: (usize, usize)) -> (usize, usize) {
        (Self::unit_count(cap.0), Self::unit_count(cap.1))
    }
}

impl fmt::Display for MemoryLoadUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.a)
    }
}

impl MemoryLoad {
    pub fn new() -> MemoryLoadBuilder {
        MemoryLoadBuilder {
            delay_start: (Duration::from_secs(0), Duration::from_secs(1)),
            mem_range: MemoryLoadUnit::unit_count_range((100, 500)),
            run_range: (Duration::from_secs(60), Duration::from_secs(120)),
        }
    }

    pub fn run(&self) {
        let mut rng = rand::thread_rng();
        let ds = rng.gen_range(self.delay_start.0..self.delay_start.1);
        thread::sleep(ds);
        let cap = rng.gen_range(self.mem_range.0..self.mem_range.1);
        let t_t = rng.gen_range(self.run_range.0..self.run_range.1);
        let mut m = Vec::with_capacity(cap);

        println!(
            "mem util:{}MB",
            cap * size_of::<MemoryLoadUnit>() / RADIX / RADIX
        );

        for i in 0..cap {
            m.push(MemoryLoadUnit::new(i as i64));
        }
        thread::sleep(t_t);
    }
}

impl MemoryLoadBuilder {
    pub fn delay_start(&mut self, dt: (u64, u64)) -> &mut Self {
        self.delay_start = (Duration::from_secs(dt.0), Duration::from_secs(dt.1));
        self
    }
    pub fn mem_range(&mut self, mem: (usize, usize)) -> &mut Self {
        self.mem_range = MemoryLoadUnit::unit_count_range(mem);
        self
    }
    pub fn run_range(&mut self, run: (u64, u64)) -> &mut Self {
        self.run_range = (Duration::from_secs(run.0), Duration::from_secs(run.1));
        self
    }
    pub fn build(&self) -> MemoryLoad {
        MemoryLoad {
            delay_start: self.delay_start,
            // MB
            mem_range: self.mem_range,
            // second
            run_range: self.run_range,
        }
    }
}

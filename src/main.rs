use clap::{App, Arg};
use std::thread;
use std::{fs::File, io::Read};
use yaml_rust::YamlLoader;

mod cpu;
mod memory;

fn main() -> std::io::Result<()> {
    // let a = memory::MemoryLoad::new()
    //     .delay_start(1)
    //     .mem_range((1024, 2048))
    //     .run_range((10, 20))
    //     .build();

    // a.run();

    // let b = cpu::CpuLoad::new()
    //     .delay_start(1)
    //     .cpu_range((30, 50))
    //     .run_range((20, 30))
    //     .build();

    // b.run();

    let matches = App::new("Random load For Kubernetes")
        .version("1.0")
        .author("Charstal <charstal_h@outlook.com>")
        .about("simulate general pod request(cpu & memory) in kubernets ")
        // .subcommand(SubCommand::with_name("cpu"))
        // .subcommand(SubCommand::with_name("memory"))
        // .subcommand(SubCommand::with_name("mix"))
        // .subcommand(SubCommand::with_name("random"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                // .about("Set a custom config file")
                .takes_value(true),
        )
        .get_matches();

    if let Some(config_file) = matches.value_of("config") {
        println!("{}", config_file);
        load_file(config_file);
    }

    Ok(())
}

fn load_file(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let mut tvec = vec![];
    for item in docs {
        println!("{}", item["mod"].as_str().unwrap());

        let t1 = thread::spawn(move || match item["mod"].as_str() {
            Some("cpu") => {
                let ds = (
                    item["delayStart"][0].as_i64().unwrap() as u64,
                    item["delayStart"][1].as_i64().unwrap() as u64,
                );
                let cr = (
                    item["cpuRange"][0].as_i64().unwrap() as usize,
                    item["cpuRange"][1].as_i64().unwrap() as usize,
                );
                let rr = (
                    item["runRange"][0].as_i64().unwrap() as u64,
                    item["runRange"][1].as_i64().unwrap() as u64,
                );
                let b = cpu::CpuLoad::new()
                    .delay_start(ds)
                    .cpu_range(cr)
                    .run_range(rr)
                    .build();

                b.run();
            }
            Some("memory") => {
                let ds = (
                    item["delayStart"][0].as_i64().unwrap() as u64,
                    item["delayStart"][1].as_i64().unwrap() as u64,
                );
                let mr = (
                    item["memRange"][0].as_i64().unwrap() as usize,
                    item["memRange"][1].as_i64().unwrap() as usize,
                );
                let rr = (
                    item["runRange"][0].as_i64().unwrap() as u64,
                    item["runRange"][1].as_i64().unwrap() as u64,
                );
                let b = memory::MemoryLoad::new()
                    .delay_start(ds)
                    .mem_range(mr)
                    .run_range(rr)
                    .build();

                b.run();
            }
            Some("random") => {}
            _ => panic!("not support"),
        });

        tvec.push(t1);
    }

    for t in tvec {
        t.join().unwrap();
    }
}

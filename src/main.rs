use std::thread;
extern crate dirs;
mod deserialize;
mod utils;
mod core;

fn main() {
    
    let programs:Vec<deserialize::Programs> = serde_json::from_reader(utils::get_db_path())
    .expect("error while reading");
    
    let mut threads = Vec::new();

    for program in programs {
        //println!("Bin {} args {} description{}", program.bin, program.args, program.description);
        let mut args = vec![];
        for arg in program.args{
            let str_arg: &str = deserialize::string_to_static_str(arg);
            args.push(str_arg);
        }
        let handler = thread::spawn(move || {
            let bin_args = args.as_slice();
            core::new_process(&program.bin, &bin_args); 
        });
        threads.push(handler);
    }

    for handler in threads {
        handler.join().unwrap();
    }
}
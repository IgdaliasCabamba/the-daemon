use std::process::Command;
use termimad::*;
use crossterm::style::Color;
use chrono::{Timelike, Utc};
use crate::core::minimad::TextTemplate;

pub fn new_process(bin:&str, args:&[&str]) -> std::process::ExitStatus{
    let now = Utc::now();

    let hour = now.hour();
    println!(
        "Starting at {:02}:{:02}:{:02}",
        hour,
        now.minute(),
        now.second(),
    );

    let base_excpetion_msg:String = "Failed to load ".to_owned();
    let excpetion_log = base_excpetion_msg + bin;

    let process = Command::new(bin)
        .args(args)
        //.status()
        .output()
        .expect(&excpetion_log);
    
    let out = std::string::String::from_utf8(process.stdout);
    println!("{:?}", out);
    
    let mut output = MadSkin::default();//no_style();
    
    if process.status.success() {
        //println!("ran {} PID {}", bin, status.code().unwrap());
        output.bold.set_fg(crossterm::style::Color::Yellow);
        output.italic.set_fg(crossterm::style::Color::Blue);
        output.paragraph.set_fg(crossterm::style::Color::Green);

        let text_template = TextTemplate::from(r#"
        * ran: **${program}**
        * code: **${status-code}**
        * *args*: **${program-args} **
        "#);

        let mut expander = text_template.expander();
        let status_code = process.status.code().unwrap().to_string();
        let mut bin_args = String::new();
        
        for i in args{
            let a:&str = i.as_ref();
            bin_args.push_str(a);
            bin_args.push_str("\n\t");
        }

        expander
            .set("program", bin)
            .set("status-code", &status_code)
            .set("program-args", &bin_args);
        output.print_expander(expander);
    }
    else{
        //println!("Failed to execute {}", bin);
        
        output.bold.set_fg(crossterm::style::Color::Red);
        output.italic.set_fg(crossterm::style::Color::Grey);

        let text_template = TextTemplate::from(r#"**Failed to execute**: *${program}*"#);
        let mut expander = text_template.expander();

        expander
            .set("program", bin);
        output.print_expander(expander);
    }
    println!(
        "Finishing at {:02}:{:02}:{:02}",
        hour,
        now.minute(),
        now.second(),
    );

    return process.status;
}
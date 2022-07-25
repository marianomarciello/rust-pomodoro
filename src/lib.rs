use std::{thread, time};
use std::error::Error;
use time::Duration;
use clap::{Arg, App};

pub struct Config {
    pub num_pomodoro: u32,
    pub dur_pomodoro: u32,
    pub dur_pause: u32,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {

        let app_name = "pomodoro";

        let matches = App::new(app_name)
            .version("0.1.0")
            .author("Mariano Marciello")
            .about("A minimal pomodoro timer")
            .arg(Arg::with_name("pomo")
                     .short("t")
                     .long("pomo")
                     .takes_value(true)
                     .help("Number of pomodoro"))
            .arg(Arg::with_name("dur")
                     .short("d")
                     .long("dur")
                     .takes_value(true)
                     .help("Duration of each pomodoro"))
            .arg(Arg::with_name("pause")
                     .short("p")
                     .long("pause")
                     .takes_value(true)
                     .help("Duration of each pause"))
            .get_matches();


        let num_pomodoro = matches.value_of("pomo");
        let num_pomodoro = match num_pomodoro {
            Some(pomo) => {
                match pomo.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => return Err("Error parsing number of pomodoros")
                }
            }
            None => return Err("Error parsing number of pomodoros")
        };

        let dur_pomodoro = matches.value_of("dur");
        let dur_pomodoro = match dur_pomodoro {
            Some(dur) => {
                match dur.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => return Err("Error parsing duration of pomodors")
                }
            }
            None => return Err("Error parsing duration of pomodoros")
        };

        let dur_pause = matches.value_of("pause");
        let dur_pause = match dur_pause {
            Some(pause) => {
                match pause.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => return Err("Error parsing duration of puase")
                }
            }
            None => return Err("Error parsing duration of pause")
        };
        Ok(Config {
                num_pomodoro,
                dur_pomodoro,
                dur_pause,
         })

    }
}

pub fn timer(time : u32, pomo: u32) {
    // start timer
    let duration = Duration::from_secs(1);
    let mut elapsed_sec = 0;
    let mut elapsed_min = 0;
    for _t in 0..time*60{
        print!("{esc}c", esc = 27 as char);
        println!("Pomodoro n° {}", pomo);
        println!("Elapsed min:{} sec:{}", elapsed_min, elapsed_sec);
        elapsed_sec += 1;
        elapsed_min += (elapsed_sec/60) % 60;
        elapsed_sec = elapsed_sec % 60;
        thread::sleep(duration);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("Start focus");
    for pomo in 0..config.num_pomodoro {
        timer(config.dur_pomodoro, pomo);

        println!("Great job take a break!!");

        timer(config.dur_pause, pomo);

        if pomo != 0 {
            println!("Go back to work!!");
        }
    }

    Ok(())
}

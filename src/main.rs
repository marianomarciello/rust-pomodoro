use std::process;
use pomodoro::Config;

fn main() {
    let app_name = "pomodoro";
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("try {} --help", app_name);
        process::exit(1);
    });

    println!("N° pomodoro: {}, during: {} minutes, with pauses of: {} minutes",
             config.num_pomodoro,
             config.dur_pomodoro,
             config.dur_pause);

    if let Err(e) = pomodoro::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}

# Intro
Just a simple pomodoro-timer cli tool written in Rust.

## Installation
```sh
cargo build --release
```
## Usage
```sh
pomodoro 0.1.0
Mariano Marciello
A minimal pomodoro timer

USAGE:
    pomodoro [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dur <dur>        Duration of each pomodoro
    -p, --pause <pause>    Duration of each pause
    -t, --pomo <pomo>      Number of pomodoro
```

## Imporvement
At the end of every pomodoro, ring a bell or do a `notify-send`.

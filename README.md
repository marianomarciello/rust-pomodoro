# pomodoro-rust

This is just an example of how the same thing (a pomodoro timer) can be done in
several different way in rust.

## pomodoro-cmd

A command-line version of a pomodoro timer.

### Usage
```sh
pomodoro-cmd
A minimal pomodoro timer.

USAGE:
    pomodoro [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dur <dur>        Duration of each pomodoro (in minutes)
    -p, --pause <pause>    Duration of each pause (in minutes)
    -t, --pomo <pomo>      Number of pomodoro
```

## pomodoro-gui

A gui version of a pomodoro timer. The gui is based on
[https://iced.rs/](iced).

![Pomodoro Gui](./docs/img/pomodoro-gui.png?raw=true)

## pomodoro-tui
A gui version of a pomodoro timer. The gui is based on
[https://docs.rs/ratatui/latest/ratatui/](ratatui).

![Pomodoro Tui](./docs/img/pomodoro-tui.png?raw=true)

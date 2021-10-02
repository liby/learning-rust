use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    time::Duration,
};

use clap::{crate_version, App, Arg};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Result,
};

mod game;

fn main() -> Result<()> {
    let matches = App::new("CLI Game of Life")
        .version(crate_version!())
        .author("Bryan")
        .about("Simple implementation of Conway's Game Of Life in Rust.")
        .after_help("Have fun!")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to configure initial state of game")
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DELAY")
                .help("Sets the delay between game ticks. Value is in miliseconds")
                .short("d")
                .long("delay")
                .takes_value(true)
                .takes_value(true)
                .default_value("500"),
        )
        .get_matches();

    let mut stdout = stdout();
    let delay: u64 = match matches.value_of("DELAY").unwrap().parse() {
        Ok(val) => val,
        Err(e) => {
            println!("Error parsing DELAY argument: {}", e);
            500
        }
    };
    // let mut game = game::Universe::new(5, 5);
    // game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    let mut game = match matches.value_of("INPUT") {
        Some(path) => create_game_from_file(path),
        None => {
            let mut default_game = game::Universe::new(5, 5);
            default_game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
            default_game
        }
    };

    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::Magenta),
        Hide
    )?;

    loop {
        queue!(stdout, Clear(ClearType::All))?;
        let mut i = 0;
        while let Some(line) = game.row_as_string(i) {
            queue!(stdout, MoveTo(0, i as u16), Print(line))?;
            i += 1;
        }

        queue!(
            stdout,
            MoveTo(0, (i + 1) as u16),
            Print("Press Esc to exit...")
        )?;
        stdout.flush()?;
        if poll(Duration::from_millis(delay))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        game.tick();
    }
    execute!(
        stdout,
        ResetColor,
        EnableLineWrap,
        Show,
        LeaveAlternateScreen
    )?;
    disable_raw_mode()?;
    Ok(())
}

fn create_game_from_file(path: &str) -> game::Universe {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut rows_number = 0;
    if let Ok(success) = reader.read_line(&mut line) {
        if success > 0 {
            rows_number = line.trim().parse().unwrap();
            line.clear();
        } else {
            panic!("Rows number not detected!");
        }
    };
    let mut cols_number = 0;
    if let Ok(success) = reader.read_line(&mut line) {
        if success > 0 {
            cols_number = line.trim().parse().unwrap();
            line.clear();
        } else {
            panic!("Cols number not detected!");
        }
    };

    let mut game_universe = game::Universe::new(rows_number, cols_number);
    let mut row = 0;
    let mut live_cells = Vec::<(u32, u32)>::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let mut col = 0;
                for char in line.chars() {
                    match char {
                        '1' => live_cells.push((row, col)),
                        _ => {}
                    }
                    col += 1;
                }
            }
            _ => break,
        }
        line.clear();
        row += 1;
    }
    game_universe.set_cells(&live_cells);
    game_universe
}

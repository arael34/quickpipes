mod types;
mod parse;

use std::{
    io::{self, stdout, Read, Write},
    thread::sleep,
    time::Duration
};
use termion::{
    style,
    clear,
    color,
    terminal_size,
    cursor::{self, Hide, Show},
    raw::IntoRawMode, async_stdin,
};
use types::Point;
use parse::{Config, TrailKind};

fn main() -> io::Result<()>{
    let stdout = stdout();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    // terminal bounds and vec of points
    let bounds:(u16, u16) = terminal_size().unwrap();

    let config = match Config::parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let number = config.number.unwrap_or(5);
    let trails = config.trailkind.unwrap_or(TrailKind::Default).into_chars();
    let bold = config.bold.unwrap_or(false);
    let delay = config.delay.unwrap_or(30);
    let turnchance = config.turnchance.unwrap_or(13);

    // Clear screen and hide cursor
    if bold { write!(stdout, "{}{}{}", clear::All, style::Bold, Hide)?; }
    else { write!(stdout, "{}{}", clear::All, Hide)?; }    

    let mut pv: Vec<Point> = Point::rand_init(number, &bounds);

    let s: u32 = bounds.0 as u32 * bounds.1 as u32 * 3 / 4;
    let mut n: u32 = 0;

    let mut active: usize = 0;

    // event loop
    loop {
        // if user presses 'q', end program
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }

        // Flush the output to the terminal
        stdout.flush()?;

         // Print and step point
         let point = &mut pv[active as usize];
         let ch = trails[(&point.direction.0).get_u8() as usize]
             [(&point.direction.1).get_u8() as usize];
         write!(
             stdout,
             "{}{}{}",
             cursor::Goto(point.pos.0, point.pos.1),
             color::Fg(point.color.as_ref()),
             ch
         )?;
         if point.step(&bounds, turnchance) { active += 1; }
         if active as u8 >= number { active = 0; }
         
         // pause
         sleep(Duration::from_millis(delay));
         n += 1;
 
         // reset terminal after a certain number of prints
         if n < s { continue; }
         write!(stdout, "{}", clear::All)?;
         pv = Point::rand_init(number, &bounds);
         n = 0;
    }

    write!(stdout, "{}{}{}{}", clear::All, style::Reset, Show, cursor::Goto(1, 1))?;
    stdout.flush().expect("Failed to clean up.");

    Ok(())
}

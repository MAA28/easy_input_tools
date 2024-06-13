use std::io::{stdin, stdout, Result, Write};

use colored::ColoredString;
use crossterm::{cursor, terminal, ExecutableCommand};

pub fn input(prompt: ColoredString, condition: impl Fn(&str) -> bool) -> Result<String> {
    let mut stdout = stdout();

    let mut input = String::new();
    loop {
        stdout.write(prompt.as_bytes())?;
        stdout.flush()?;
        stdin().read_line(&mut input)?;
        stdout.execute(cursor::Hide)?;
        input.pop();
        if condition(&input) {
            break;
        }
        stdout.execute(cursor::MoveUp(prompt.lines().count() as u16))?;
        stdout.execute(cursor::MoveToColumn(0))?;
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        stdout.execute(cursor::Show)?;
        input.clear();
    }

    Ok(input)
}

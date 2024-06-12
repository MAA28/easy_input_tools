use std::io::{stdout, Result, Write};

use colored::ColoredString;
use crossterm::{cursor, terminal, ExecutableCommand};
use k_board::{keyboard::Keyboard, keys::Keys};

pub fn options(
    options: &Vec<&str>,
    format_option: impl Fn(&str, usize, usize, bool) -> ColoredString,
) -> Result<usize> {
    let mut stdout = stdout();

    stdout.execute(cursor::Hide)?;

    let mut selected = 0;

    let mut output = render(options, selected, &format_option, false);

    'selection: loop {
        stdout.write(output.as_bytes())?;
        for key in Keyboard::new() {
            match key {
                Keys::Down => {
                    selected = (selected + options.len() + 1) % options.len();
                    break;
                }
                Keys::Up => {
                    selected = (selected + options.len() - 1) % options.len();
                    break;
                }
                Keys::Enter => break 'selection,
                _ => {}
            }
        }
        stdout.execute(cursor::MoveUp(output.lines().count() as u16 - 1))?;
        stdout.execute(cursor::MoveToColumn(0))?;
        output = render(options, selected, &format_option, false);
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    }

    stdout.execute(cursor::MoveUp(output.lines().count() as u16 - 1))?;
    stdout.execute(cursor::MoveToColumn(0))?;
    stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    output = render(options, selected, &format_option, true);
    stdout.write(output.as_bytes())?;
    stdout.execute(cursor::Show)?;
    Ok(selected)
}

fn render(
    options: &[&str],
    selected: usize,
    format_option: &impl Fn(&str, usize, usize, bool) -> ColoredString,
    last: bool,
) -> ColoredString {
    let binding = options
        .iter()
        .enumerate()
        .map(|(i, option)| format_option(option, i, selected, last))
        .fold(String::new(), |acc, f| format!("{}\n{}", acc, f));
    binding.strip_prefix("\n").unwrap().into()
}

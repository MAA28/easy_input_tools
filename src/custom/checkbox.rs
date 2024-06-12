use std::collections::HashSet;
use std::io::{stdout, Result, Write};

use colored::ColoredString;
use crossterm::{cursor, terminal, ExecutableCommand};
use k_board::keyboard::Keyboard;
use k_board::keys::Keys;

pub fn checkbox(
    boxes: &Vec<&str>,
    format_option: impl Fn(&str, usize, usize, &HashSet<usize>) -> ColoredString,
) -> Result<HashSet<usize>> {
    let mut stdout = stdout();

    stdout.execute(cursor::Hide)?;

    let mut selected = 0;
    let mut selection = HashSet::new();

    let mut output = render(boxes, selected, &selection, &format_option);

    'selection: loop {
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        stdout.write(output.as_bytes())?;

        for key in Keyboard::new() {
            match key {
                Keys::Down => {
                    selected = (selected + boxes.len() + 1) % boxes.len();
                    break;
                }
                Keys::Up => {
                    selected = (selected + boxes.len() - 1) % boxes.len();
                    break;
                }
                Keys::Space => {
                    if selection.contains(&selected) {
                        selection.remove(&selected);
                    } else {
                        selection.insert(selected);
                    }
                    break;
                }
                Keys::Enter => break 'selection,
                _ => {}
            }
        }
        stdout.execute(cursor::MoveUp(output.lines().count() as u16 - 1))?;
        stdout.execute(cursor::MoveToColumn(0))?;
        output = render(boxes, selected, &selection, &format_option);
    }

    output = render(boxes, boxes.len(), &selection, &format_option);

    stdout.execute(cursor::MoveUp(output.lines().count() as u16 - 1))?;
    stdout.execute(cursor::MoveToColumn(0))?;
    stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    stdout.write(format!("{}\n", output).as_bytes())?;
    stdout.execute(cursor::Show)?;
    Ok(selection)
}

fn render(
    boxes: &Vec<&str>,
    selected: usize,
    selection: &HashSet<usize>,
    format_option: &impl Fn(&str, usize, usize, &HashSet<usize>) -> ColoredString,
) -> ColoredString {
    let binding = boxes
        .iter()
        .enumerate()
        .map(|(i, option)| format_option(option, i, selected, &selection))
        .fold(String::new(), |acc, f| format!("{}\n{}", acc, f));
    binding.strip_prefix("\n").unwrap().into()
}

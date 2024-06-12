use std::io::Result;
use colored::Colorize;

pub fn options(options: &Vec<&str>) -> Result<usize> {
    crate::custom::options(options, |option, i, selected, last| {
        let output = format!(" {} {}", if i == selected { "◉︎" } else { "○︎" }, option);
        if i == selected && !last {
            output.bold()
        } else {
            output.into()
        }
    })
}

use std::io::Result;

use colored::ColoredString;

pub fn input(prompt: ColoredString, condition: impl Fn(&str) -> bool) -> Result<String> {
    crate::custom::input(format!("{}\n> ", prompt).into(), condition)
}

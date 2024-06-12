use std::collections::HashSet;
use std::io::Result;

use colored::Colorize;

pub fn checkbox(options: &Vec<&str>) -> Result<HashSet<usize>> {
    crate::custom::checkbox(options, |option, i, selected, selection| {
        let output = format!(
            " {} {}",
            if selection.contains(&i) { "☑" } else { "☐" },
            option,
        );
        if i == selected {
            output.bold()
        } else {
            output.into()
        }
    })
}

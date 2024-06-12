use std::collections::HashSet;
use std::io::Result;


pub fn checkbox(options: &Vec<&str>) -> Result<HashSet<usize>> {
    crate::custom::checkbox(
        options,
        |option, i, selected, selection| {
            format!(
                "{}{}{} {}",
                if i == selected { ">" } else { "[" },
                if selection.contains(&i) { "x" } else { " " },
                if i == selected { "<" } else { "]" },
                option,
            ).into()
        },
    )
}

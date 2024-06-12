use std::io::Result;

pub fn options(options: &Vec<&str>) -> Result<usize> {
    crate::custom::options(options, |option, i, selected, _| {
        format!("({}) {}", if i == selected { "x" } else { " " }, option).into()
    })
}

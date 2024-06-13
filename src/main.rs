use colored::Colorize;

mod custom;
mod ascii;
mod unicode;


fn main() {
    let s = custom::input("Enter your name\n> ".bold(), |input| !input.is_empty());
    let x = unicode::checkbox(&vec!["a", "b", "c", "d", "e"]);
    let y = unicode::options(&vec!["a", "b", "c", "d", "e"]);
}

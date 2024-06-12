mod custom;
mod ascii;
mod unicode;


fn main() {
    let x = unicode::checkbox(&vec!["a", "b", "c", "d", "e"]);
    let y = unicode::options(&vec!["a", "b", "c", "d", "e"]);
}


fn main() {
    println!("Do you like to code in javascript?");
    let options = vec!["yes", "no"];
    if let Ok(awnser_index) = easy_input_tools::unicode::options(&options) {
        if awnser_index == 0 {
            println!("Wow... be careful what you say!");
        } else {
            println!("Right on");
        }
    } else {
        println!("Some io issue");
    }
}


fn main() {
    println!("What colors do you like?");
    let options = vec!["red", "blue", "green"];
    if let Ok(colors_indecies) = easy_input_tools::unicode::checkbox(&options) {
        println!(
            "You like: {:?}",
            options
                .iter()
                .enumerate()
                .filter(|(i, _)| colors_indecies.contains(i))
                .map(|(_, option)| option)
                .collect::<Vec<_>>()
        );
    } else {
        println!("Some io issue");
    }
}

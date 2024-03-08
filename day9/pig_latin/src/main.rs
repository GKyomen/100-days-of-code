use std::io;

fn main() {
    println!("Input a word to see it in pig latin!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if vowels.iter().any(|&v| input.starts_with(v)) {
        println!(
            "The word {} in pig latin is {}",
            input,
            format!("{}-hay", input)
        );
        return;
    }

    let (first, rest) = input.split_at(1);
    println!(
        "The word {} in pig latin is {}",
        input,
        format!("{}-{}ay", rest, first)
    );
}

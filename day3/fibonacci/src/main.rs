use std::io;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("Welcome to the Fibonacci Calculator in Rust!");

    loop {
        println!("Enter the index of the fibonacci sequence that you wish to know:");

        let mut n = String::new();
        
        io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    
        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a whole posive number.");
                continue;
            },
        };

        println!("The {}th fibonacci number is {}", n, fibonacci(n));
        break;
    }
}

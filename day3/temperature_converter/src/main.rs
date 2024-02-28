use std::io;

fn main() {
    println!("Welcome to the Temperature Converter in Rust!");

    loop {
        println!("Menu. Input one of the following options:");
        println!("0 - Exit the program");
        println!("1 - Convert ºC to ºF");
        println!("2 - Convert ºF to ºC");   

        let mut option = String::new();
    
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
    
        let option: u32 = match option.trim().parse() {
            Ok(num) => {
                if num > 2 {
                    println!("Please enter exactly one of the options. Returning to the menu.");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please enter exactly one of the options. Returning to the menu.");
                continue;
            },
        };

        let value_to_convert: f32 = match option {
            0 => {
                println!("Thanks for using this program. Bye!");
                break;
            }
            1 | 2 => {
                loop {
                    println!("Now enter the value to be converted (number, whole or fractional):");

                    let mut value = String::new();

                    io::stdin()
                        .read_line(&mut value)
                        .expect("Failed to read line");

                    let value: f32 = match value.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a number, whole or fractional.");
                            continue;
                        },
                    };

                    break value;
                }
            }
            _ => {
                println!("Unexpected error. Returning to menu");
                continue;
            }
        };
        
        match option {
            1 => {
                let converted = value_to_convert * 1.8 + 32.0;
                println!("");
                println!("Conversion result: {}°C is {}°F", value_to_convert, converted);
                println!("");
                println!("Returning to menu");
                continue;
            }
            2 => {
                let converted = (value_to_convert - 32.0) / 1.8;
                println!("");
                println!("Conversion result: {}°F is {}°C", value_to_convert, converted);
                println!("");
                println!("Returning to menu");
                continue;
            }
            _ => {
                println!("Unexpected error. Returning to menu");
                continue;
            }
        };
    }
}

// fn main() {
//     let v = vec![100, 32, 57];
//     for n_ref in &v {
//         // n_ref has type &i32
//         let n_plus_one: i32 = *n_ref + 1;
//         println!("{n_plus_one}");
//     }
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
//     println!("Score: {}", score);

//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
// }

// fn main() {
//     use std::collections::HashMap;

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // field_name and field_value are invalid at this point, try using them and
//     // see what compiler error you get!
//     println!("{}", field_name);
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);

//     println!("{:?}", scores);
// }

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);

//     println!("{:?}", scores);
// }

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
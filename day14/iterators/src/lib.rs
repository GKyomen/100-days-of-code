#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

// ! it works but doesn't make sense. When you compare shoes with a size,
// ! you expect to get the shoes, not a reference to them. The original code
// ! with into_iter and passing ownership makes more sense.
pub fn shoes_in_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(&shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                &Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                &Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
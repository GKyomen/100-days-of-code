fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter().map(|x| x * 2).map(|x| x + 1);

    for val in v1_iter {
        println!("Got: {}", val);
    }
    
    let v2 = vec![1, 2];
    let test1 = v2.iter();
    let test2 = v2.iter();
    for val1 in test1 {
        // ! doesn't compile no matter the type of vec values
        // ! because iter is only able to run once (it is consumed)
        for val2 in test2 {
            println!("Got: {} & {}", val1, val2);
        }
    }
}
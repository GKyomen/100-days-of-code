// fn incr(n: &mut i32) {
//     *n += 1;
// }

// fn main() {
//     let mut n = 1;
//     incr(&n); // ! Não compila pq 'n' é mutável, mas &n não. Teria que passar uma referência mutável (&mut n)
//     println!("{n}");
// }

// fn main() {

//     let mut s = String::from("hello");
  
//     let s2 = &s;
  
//     let s3 = &mut s; // ! Não compila pq 's' já foi emprestada, perdendo a permissão de empréstimo
  
//     s3.push_str(" world");
  
//     println!("{s2}");
  
// }

// fn main() {
//     let mut s = String::from("hello");

//     for &item in s.as_bytes().iter() { // ! Não compila pq tem dois empréstimos. Um aqui (imutável)

//         if item == b'l' {

//         s.push_str(" world"); // ! Outro aqui (mutável)

//         }

//     }

//     println!("{s}");
// }

fn main() {
    println!(
      "&String={} &str={}",
      std::mem::size_of::<&String>(), // Menor por ser ponteiro normal
      std::mem::size_of::<&str>(), // "Fat" pointer -> tem o ponteiro + metadata (nesse caso o len da string)
    );
}
fn main() {
    for i in 0..11 {
        let word = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} {}", word, i);
    }
}
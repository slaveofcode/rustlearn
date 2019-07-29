fn modifies(x: &mut f64) {
    *x = 1.0
}

fn main() {
    let mut val = 18.5;
    modifies(&mut val);
    println!("{}", val);
}
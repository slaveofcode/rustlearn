fn sqr(x: f64) -> f64 {
    x * x
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);

    let res_abs = abs(-3.0);
    println!("absolute of -3.0 is {}", res_abs);

    let res_clamp = clamp(4.0, 2.0, 3.0);
    println!("clamp value is {}", res_clamp);
}
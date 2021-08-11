fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if (y - x).abs() > 0.1 {
        println!("Success!");
    }
}

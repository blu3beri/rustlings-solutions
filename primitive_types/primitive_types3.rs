fn main() {
    let mut a = vec![];
    for x in 0..100 {
        a.push(x)
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}

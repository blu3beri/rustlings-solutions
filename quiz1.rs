fn calculate_apple_price(mut order_amount: i32) -> i32 {
    if order_amount > 40 {
       return order_amount;
    }
    order_amount * 2
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}

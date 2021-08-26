use std::io;

fn fee(price: f64) -> f64 {
    // fee is %0.010
    price * 0.001
}

fn read_value(msg: &str) -> f64 {
    let mut buffer = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f64>().unwrap()
}

fn main() {
    let amount: f64 = read_value("Enter currency amount:");
    let purchase: f64 = read_value("Enter purchase price:");
    let sell: f64 = read_value("Enter sell price:");
    let profit: f64 = (amount * sell - amount * purchase) - fee(amount * sell);
    println!("==================================");
    println!("Amount = {0}", amount);
    println!("Purchase price = {0}", purchase);
    println!("Sell price = {0}", sell);
    println!("Fee = {}", fee(sell));
    println!("Profit = {}", profit);
}

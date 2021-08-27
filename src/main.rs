use console::style;
use dialoguer::{theme::ColorfulTheme, Input};

fn fee(price: f64) -> f64 {
    // fee is %0.010
    price * 0.001
}

fn read_value(msg: &str) -> f64 {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact_text()
        .unwrap();
    input.trim().parse::<f64>().unwrap()
}

fn main() {
    loop {
        let amount: f64 = read_value("Enter currency amount (BTC):");
        let purchase: f64 = read_value("Enter purchase price ($):");
        let sell: f64 = read_value("Enter sell price ($):");
        let fee_val: f64 = fee(amount * sell);
        let profit: f64 = (amount * sell - amount * purchase) - fee_val;
        println!("==================================");
        println!("Amount = {0} BTC", amount);
        println!("Purchase price = ${0}", purchase);
        println!("Sell price = ${0}", sell);
        println!("Fee = ${}", style(fee_val).yellow());
        println!("Profit = ${}", style(profit).green());
        println!("==================================");
    }
}

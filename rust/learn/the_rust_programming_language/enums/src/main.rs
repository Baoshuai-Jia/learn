fn main() {}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_inf_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

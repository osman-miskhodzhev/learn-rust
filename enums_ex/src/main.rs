fn main() {
    enum Coin {
        One,
        Two,
        Five,
        Ten,
    }
    fn hello_coin(coin: Coin) -> u8 {
        match coin {
            Coin::One => 1,
            Coin::Two => 2,
            Coin::Five => 5,
            Coin::Ten => 10,
        }
    }
    
    println!("{}", hello_coin(Coin::One));
}


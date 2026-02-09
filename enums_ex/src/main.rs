fn main() {
    enum Coin {
        One,
        Two,
        Five,
        Ten,
    }
    fn hello_coin(coin: Coin) {
        match coin {
            Coin::One => {println!("1 рубль");},
            Coin::Two => {println!("2 рубля");},
            Coin::Five => {println!("5 рублей");},
            Coin::Ten => {println!("10 рублей");},
        }
    }
    
    hello_coin(Coin::One);
    hello_coin(Coin::Two);
    hello_coin(Coin::Five);
    hello_coin(Coin::Ten);
    
    enum TheBill {
        Five,
        Ten,
        Fifty,
    }
    fn hello_bill(bill: TheBill) {
        match bill {
            TheBill::Five => {println!("Малютка 5-ти рублевая");},
            TheBill::Ten => {println!("Классика");},
            TheBill::Fifty => {println!("50 рублей");},
        }
    }
    hello_bill(TheBill::Five);
    hello_bill(TheBill::Ten);
    hello_bill(TheBill::Fifty);
}


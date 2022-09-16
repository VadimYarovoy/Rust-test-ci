fn main() {
    println!("{}", value_in_rubbles(Coin::Rubble))
}

enum Coin{
    Rubble,
    Dollar,
    Euro,
}

fn value_in_rubbles(value: Coin) -> u8 {
    match value {
        Coin::Rubble => {
            println!("Rubble side");
            1
        }
        Coin::Dollar => 70,
        Coin::Euro => 80,
    }
}
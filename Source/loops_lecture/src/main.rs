fn main() {
    let mut leaves: String = String::from("*");
    let mut air: String = String::from("     ");

    let mut lvl: u8 = 0;

    while lvl != 5 {
        println!("{air}{leaves}");
        leaves.push_str("**");
        air.remove(0);
        lvl += 1;
    }
    println!("     |");
}

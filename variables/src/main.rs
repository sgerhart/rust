const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 1;

fn main() {
    let (missiles, ready, _go) = (STARTING_MISSILES, READY_AMOUNT, 3);

    println!("Firing {} or my {} missiles...", ready, missiles);

    //missiles = missiles - ready;

    println!("{} missiles left", missiles - ready);
}

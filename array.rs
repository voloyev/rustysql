fn main() {
    let arr = [10, 20, 30, 40];
    let first = &arr.get(0);
    let last = &arr.get(5);
    println!("First {:?}", first);
    println!("Last {:?}", last)
}

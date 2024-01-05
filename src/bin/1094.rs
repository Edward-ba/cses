use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let length: usize = input.trim().parse().expect("Please type a number!");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut arr: Vec<u64> = input.trim().split(" ")
        .map(|x: &str| x.parse::<u64>().unwrap()).collect();
    let mut total: u64 = 0;
    for i in 1..length {
        if arr[i] < arr[i - 1] {
            total += arr[i - 1] - arr[i];
            arr[i] = arr[i - 1];
        }
    }
    println!("{}", total)
}
use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let length: u32 = input.trim().parse().expect("Please type a number!");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut arr: Vec<u32> = input.trim().split(" ")
        .map(|x: &str| x.parse::<u32>().unwrap()).collect();
    
    arr.sort();

    for i in 0..length + 1 {
        if arr.len() == i as usize {
            println!("{}", i + 1);
            break;
        }
        if arr[i as usize] != i + 1 {
            println!("{}", i + 1);
            break;
        }
    }
}

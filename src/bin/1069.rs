use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: &str = input.trim();
    let mut cur_len = 1;
    let mut max_len = 0;
    let input: Vec<_> = input.chars().collect();
    for i in 1..input.len() {
        if input[i] == input[i - 1] {
            cur_len += 1
        }
        else {
            if max_len < cur_len {
                max_len = cur_len
            }
            cur_len = 1
        }
    }
    if max_len < cur_len {
        max_len = cur_len
    }
    println!("{}", max_len)
}
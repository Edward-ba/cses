use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u64 = input.trim().parse().expect("Please type a number!");
    bruh(input)
}

fn bruh(mut n: u64){
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        }
        else {
            n *= 3;
            n += 1;
            println!("{}", n);
            n /= 2;
        }
    }
    println!("{}", n);
}

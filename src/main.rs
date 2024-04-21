use std::io;

fn fib_rec(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib_rec(n - 1) + fib_rec(n - 2);
    }
}

fn fib() {
    loop {
        println!("Please enter the position of the Fibonacci number (starting from 0) you want to find out:");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");

        let choice: i32 = match choice.trim().parse() {
            Ok(ch) => ch,
            Err(_) =>  { 
                println!("Please enter a number (starting from 0)!");
                continue;
            }
                        
        };
        
        if choice < 0 {
            println!("Wrong input!");
            continue;
        } else {
            println!("The {}-th Fibonacci number is: {}", choice, fib_rec(choice - 1));
            break;
        }
    }
}

fn main() {
    fib();
}

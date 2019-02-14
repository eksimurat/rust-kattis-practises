fn main() {
    let mut input = String::new();
    let mut number = String::new();
    let mut seq = 0;

    std::io::stdin().read_line(&mut input).expect("error reading from the input");

    match input.trim().parse::<u32>(){
        Ok(i) => 
        while seq < i {
            std::io::stdin().read_line(&mut number).expect("error reading from the input");
            match number.trim().parse::<i32>(){
                Ok(n) => if n % 2 == 0 {
                    println!("{} is even", n);
                } else {
                    println!("{} is odd", n);
                }
                Err(err) => println!("{}, {}", err, number)
            }
            number = String::new();
            seq = seq + 1;
        }
        Err(err) => println!("{}", err)
    }
}
fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("error reading from the input");

    let mut numbers = input.split_whitespace().map(|input| input.parse::<i32>());
    match(numbers.next(), numbers.next()){
        (Some(Ok(r1)), Some(Ok(s))) => {
            println!("{}", (s*2) - r1);
        }
        _ => {}
    }
}
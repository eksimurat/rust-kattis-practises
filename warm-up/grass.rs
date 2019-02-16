fn main(){
    let mut input = String::new();
    let mut sum = 0_f64;
    let mut seq = 0;


    std::io::stdin().read_line(&mut input).expect("error reading from the input");

        match input.trim().parse::<f64>(){
        Ok(price) =>  {
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("error reading from input");
        match input.trim().parse::<u32>(){
            Ok(n) => {
                while seq < n {
                input = String::new();
                std::io::stdin().read_line(&mut input).expect("error reading from input");
                let mut numbers = input.split_whitespace().map(|input| input.parse::<f64>());
                match(numbers.next(), numbers.next()){
                    (Some(Ok(num1)), Some(Ok(num2))) => {
                        sum = sum + (num1 * num2 * price)
                    }
                    _ => {}
                }
                seq = seq + 1;
                }
            }
            Err(err) => println!("{}", err)
        }
        println!("{:.32}", sum);
        }
        Err(err) => println!("{}", err)
    }


}
fn main() {
    let mut input = String::new();
    let mut number = String::new();
    let mut power = String::new();
    let mut sum = 0;
    let mut seq = 0;


    std::io::stdin().read_line(&mut input).expect("error reading from the input");

    match input.trim().parse::<u32>(){
        Ok(n) =>  {while seq < n {
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("error reading from input");
        power = input.chars().skip(input.len() - 2).take(1).collect();
        number = input.chars().take(input.len() - 2).collect();
        match power.trim().parse::<u32>(){
            Ok(p) => {match number.trim().parse::<u32>(){
                    Ok(num) => {sum = sum + num.pow(p);}
                    Err(err) => println!("{}", err)
                } 
            }
            Err(err) => println!("{}", err)
        }
        seq = seq + 1;
        }
        println!("{}", sum);
        }
        Err(err) => println!("{}", err)
    }


}
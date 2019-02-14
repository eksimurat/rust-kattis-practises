fn main(){
    let mut input_x = String::new();
    let mut input_y = String::new();

    std::io::stdin().read_line(&mut input_x).expect("error reading from the input");
    std::io::stdin().read_line(&mut input_y).expect("error reading from the input");


    match input_x.trim().parse::<i32>(){
        Ok(x) => 
        match input_y.trim().parse::<i32>(){
            Ok(y) => 
            if x < 0 {
                if y < 0 {
                    println!("{}", 3);
                } else{
                    println!("{}", 2);
                }
            } else {
                if y < 0{
                    println!("{}", 4);
                } else {
                    println!("{}", 1);
                }
            }
            Err(err) => println!("{}", err)
        }
        Err(err) => println!("{}", err)
        }
        
    }

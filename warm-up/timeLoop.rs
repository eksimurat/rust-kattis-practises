fn main(){
    let mut n = String::new();
    let mut count = 1;

    std::io::stdin().read_line(&mut n).expect("failed to read from stdin");

    match n.trim().parse::<u32>(){
        Ok(i) => while count <= i {
        println!("{} Abracadabra", count);
        count = count + 1;
        }
        Err(err) => println!("{}", err);
    }
}
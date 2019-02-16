fn main(){
    let mut input = String::new();
    let mut txt = String::new();
    
    std::io::stdin().read_line(&mut input).expect("error reading from the input");

    let vector: Vec<&str> = input.split('-').collect();

    for each in &vector {
        txt.push(each.chars().next().unwrap());
    }   

    println!("{}", txt);
}
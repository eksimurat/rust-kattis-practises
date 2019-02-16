fn main(){
    let mut input = String::new();

    let king = 1;
    let queen = 1;
    let rooks = 2;
    let bishops = 2;
    let knights = 2;
    let pawns = 8;

    std::io::stdin().read_line(&mut input).expect("error reading from the input");

    let mut set = input.split_whitespace().map(|input| input.parse::<i32>());
    match(set.next(), set.next(), set.next(), set.next(), set.next(), set.next()){
        (Some(Ok(k)), Some(Ok(q)), Some(Ok(r)), Some(Ok(b)), Some(Ok(ks)), Some(Ok(p))) => {
            println!("{} {} {} {} {} {}", king - k, queen - q, rooks - r, bishops - b, knights - ks, pawns - p)
        }
        _ => {}
    }
}
fn main() {
    let mut input = String::new();
    let mut s_count = 0;

    std::io::stdin().read_line(&mut input).expect("error reading from the input");

    for (i, c) in input.chars().enumerate(){
        if c == 's' {
            s_count = s_count + 1;
            if s_count == 2 {
                println!("hiss");
                break;
            }
        } else {
            s_count = 0;
        }
        if s_count != 2 && i + 1 == input.len(){
            println!("no hiss");
        }
    }
}
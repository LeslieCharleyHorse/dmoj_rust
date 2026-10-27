fn main() {
    // input lib
    use std::io;
    
    // var to old input
    let mut input: String = String::new();
    
    // var to int val of input
    let mut res: i32 = 0;
    
    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");
    
    // assin converted val
    res = input.trim().parse::<i32>().expect("Convert to i32 = Failed");
    
    //lesliecharleyhorse
    
    println!("{}", res*res);
}



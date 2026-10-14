fn main()
{
    // import input/output library
    use std::io;


    // var to hold input string 
    let mut input: String = String::new();

    // var to hold int val of input
    let mut int: f64 = 0.0;


    // read input, assign val to input var
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // convert string to int and assign val to int var
    int = input.trim().parse::<f64>().expect("Convert to int = Failed");

    // result
    println!("{}", (int / (5.0 / 9.0)) + 32.0)

    // lesliecharleyhorse
}



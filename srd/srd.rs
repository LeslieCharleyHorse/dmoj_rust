fn main()
{
    // import input + output library 
    use std::io;

    // var to hold result
    let mut res: i64 = 0;

    // var to hold input
    let mut input: String = String::new();


    // read input number, assign val to input var
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // assign converted string val to res
    res = input.trim().parse::<i64>().expect("Convert to i64 = Failed");

    // result
    // lesliecharleyhorse
    println!("{}", res * 2 * 4);
}



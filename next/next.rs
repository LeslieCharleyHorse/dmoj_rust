fn main()
{
    // import input read and input reader object
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();


    // var to hold input 
    let mut input: String = String::new();


    // vars to hold nums 
    let mut num1: i16 = 0;
    let mut num2: i16 = 0;


    // read first number
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // assign value to num1
    num1 = input.trim().parse::<i16>().expect("Convert to number = Failed");


    // clear input string 
    input.clear();


    // read second number
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // assign value to num1
    num2 = input.trim().parse::<i16>().expect("Convert to number = Failed");


    // find answer 
    println!("{}", num2 - num1 + num2);
    //lesliecharleyhorse






}
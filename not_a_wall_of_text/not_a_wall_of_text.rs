fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;


    // import splitwhitespace object
    use std::str::SplitWhitespace;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input string
    let mut input: String = String::new();


    // var to hold words
    let mut words: SplitWhitespace = "".split_whitespace();


    // readline
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // split input by spaces
    words = input.split_whitespace();


    // result
    //lesliecharleyhorse
    println!("{}", words.count());
}




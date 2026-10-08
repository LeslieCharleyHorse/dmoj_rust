fn main()
{
    // lesliecharleyhorse
    //import input reader object and functionality 
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // vars for months and days
    let mut month: i16 = 0;
    let mut day: i16 = 0;



    // read first number
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // assign first number
    month = input.trim().parse::<i16>().expect("Convert to int = Failed");

    // clear input
    input.clear();


    // check month
    if month > 2
        {
            println!("After");
        }

    else if month < 2
        {
            println!("Before");
        }

    else
        {
            // read second number
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // assign second number
            day = input.trim().parse::<i16>().expect("Convert to int = Failed");


            // check conditional for day
            if day > 18
                {
                    println!("After");
                }

            else if day < 18
                {
                    println!("Before");
                }

            else
                {
                    println!("Special");
                }

        }



}





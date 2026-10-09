fn main()
{
    // import input reader object and functionlaity 
    use std::io;
    use std::io::Stdin;


    // create input reader object 
    let input_reader: Stdin = io::stdin();


    // var to hold input
    let mut input: String = String::new();


    // vars to hold scores
    let mut a_score: i32 = 0;
    let mut b_score: i32 = 0;


    // calculate team a score
    for int in (1..=3).rev()
        {
            // read number
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // create temp var to hold int val
            let mut temp: i32 = 0;

            // assign number value
            temp = input.trim().parse::<i32>().expect("Readline = Failed");

            // add value * point value to score
            a_score += temp * int;

            // clear input
            input.clear();
        }



    // calculate team b score
    for int in (1..=3).rev()
        {
            // read number
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // create temp var to hold int val
            let mut temp: i32 = 0;

            // assign number value
            temp = input.trim().parse::<i32>().expect("Readline = Failed");

            // add value * point value to score
            b_score += temp * int;

            // clear input
            input.clear();
        }



        // conditionals to find answer
        if a_score > b_score
            {
                println!("A");
            }

        else if b_score > a_score
            {
                println!("B");
            }

        else
            {
                println!("T");
            }


}
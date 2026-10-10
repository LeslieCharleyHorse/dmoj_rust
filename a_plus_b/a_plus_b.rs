fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;

    // import splitwhitespace object and functionality
    use std::str::SplitWhitespace;




    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    // lesliecharleyhorse
    let mut input: String = String::new();


    // var to hold int value of input
    let mut input_int: i32 = 0;



    // read input and assign val to input var
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // assign values
    input_int = input.trim().parse::<i32>().expect("Convert to int = Failed");




    // for loop iterate for each addtion problem by parsing string to int value
    for i in 0..input_int
        {
            // var to hold res
            let mut temp_res: i32 = 0;

            // var to hold input 
            let mut temp_input: String = String::new();

            // var to hold split values
            let mut temp_nums: SplitWhitespace = "".split_whitespace();


            // readline
            input_reader.read_line(&mut temp_input).expect("Readline = Failed");


            // asign split values
            temp_nums = temp_input.split_whitespace();


            // do addtion with for loop to find result
            for num in temp_nums
                {
                    // parse numbers to int values then add to result
                    temp_res += num.parse::<i32>().expect("Convert to int = Failed");
                }

            
            // print result
            println!("{}", temp_res);

        }
}






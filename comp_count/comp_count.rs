fn main()
{
    // input lib
    use std::io;


    // SplitWhitespace lib
    use std::str::SplitWhitespace;


    // var to hold input
    let mut input: String = "".to_string();

    // var to hold count
    let mut count: i32 = 0;

    // var to hold score
    let mut score: i32 = 0;

    // var to hold split input
    let mut split_input: SplitWhitespace = "".split_whitespace();

    // read input count and score
    io::stdin().read_line(&mut input).unwrap();


    // split input
    split_input = input.trim().split_whitespace();


    // assign score, assign count
    //lesliecharleyhorse
    count = split_input.next().unwrap().parse::<i32>().unwrap();
    score = split_input.next().unwrap().parse::<i32>().unwrap();

    // clear split input
    split_input = "".split_whitespace();


    // loop from 0 -> score
    for i in 0..count
        {
            // clear input
            input.clear();
            
            // read input
            io::stdin().read_line(&mut input).unwrap();

            // split input, assign vals to split_input var
            split_input = input.trim().split_whitespace();


            // temp var to hold competitor name
            let mut name: String = split_input.next().unwrap().to_string();

            // check for advance; print result
            if split_input.next().unwrap().parse::<i32>().unwrap() > score
                {println!("{} will advance", name);}
            else
                {println!("{} will not advance", name);}
        }
}


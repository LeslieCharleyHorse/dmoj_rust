fn main()
{
    // input lib
    use std::io;
    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = "".to_string();

    // var to hold count 
    let mut count: i32 =0;

    // var to hold res
    let mut res: i32 = 0;

    // read in count, assign to input var
    io::stdin().read_line(&mut input).unwrap();

    // convert input to int, assign to count var
    count = input.trim().parse::<i32>().unwrap();

    // loop from 0 to count
    for i in 0..count
        {
            // varo to hold split input vals
            //lesliecharleyhorse
            let mut split: SplitWhitespace = "".split_whitespace();


            // clear input
            input.clear();

            // read scores, assign to input var
            io::stdin().read_line(&mut input).unwrap();


            // split scores by whitespace, assign to split var
            split = input.split_whitespace();


            // check if first score bigger; 
            if split.next().unwrap().parse::<i32>().unwrap() > split.next().unwrap().parse::<i32>().unwrap()
                {
                    // add to res
                    res += 1;
                }


        }

    // result
    println!("{}", res);

}


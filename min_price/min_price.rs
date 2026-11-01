fn main()
{
    // input lib
    use std::io;


    // var to hold input
    let mut input: String = "".to_string();

    // lesliecharleyhorse


    

    // read number count input, assign to input var
    io::stdin().read_line(&mut input).unwrap();


    // check for singular price
    if input.trim() == "1"
        {
            // clear input 
            input.clear();

            // read input price
            io::stdin().read_line(&mut input).unwrap();

            // result
            println!("{}", input.trim());
            
        }

    // multiple prices
    else
        {
            // SplitWhitespace lib
            use std::str::SplitWhitespace;

            // var to hold split input
            let mut split_input: SplitWhitespace = "".split_whitespace();

            

            // clear input
            input.clear();


            // read input prices
            io::stdin().read_line(&mut input).unwrap();


            // split input vals, assign to split_input
            split_input = input.split_whitespace();


            // var to hold minimum price created from converted int val from first input price in split input 
            let mut res: i32 = split_input.next().unwrap().parse::<i32>().unwrap();


            // loop thru split vals
            for num in split_input
                {
                    // temp var hold int val of num
                    let mut temp_num = num.parse::<i32>().unwrap();


                    // check for minimum price
                    if temp_num < res
                        {
                            // assign new min price
                            res = temp_num;
                        }

                }


            // result
            println!("{}", res);
        }

}


                
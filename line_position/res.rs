fn main()
{
    // input lib
    use std::io;

    // var to hold input
    let mut input: String = String::new();

    // var to hold count
    let mut count: i32 = 0;

    // var to hold res
    let mut res: i32 = 0;

    // var to hold shirt color 
    let mut color: String = " ".to_string();

    // read in count; assign to input var
    io::stdin().read_line(&mut input).unwrap();

    // convert input to int val, assign to count var
    //lesliecharleyhorse
    count = input.trim().parse::<i32>().unwrap();


    // loop ffom 0 to count
    for i in 0..count
        {
            // clear input
            input.clear();

            // read in shirt color, assign to input var
            io::stdin().read_line(&mut input).unwrap();

            // check if shirt color not the same as the shirt infront
            if input.trim() != color
                {
                    // add to res
                    res += 1;

                    // change current sirt color
                    color = String::from(input.trim());
                }

        }


    // result
    println!("{}", res + 1);
}


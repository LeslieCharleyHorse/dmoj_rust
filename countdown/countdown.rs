fn main()
{
    // input lib
    use std::io;
    // split lib
    use std::str::Split;

    // var to hold input
    let mut input: String = String::new();

    // var t hold splt vals
    let mut split: Split<'_, char> = "".split(' ');

    // var to hold res tuple (multiply by 60?, result)
    let mut res: (bool, i32) = (true, 0);

    // lesliecharleyhorse
    // read input -> MM:SS
    io::stdin().read_line(&mut input).unwrap();

    // split input by; :, assign val to split var
    split = input.trim().split(':');

    // loop thru split vals; minutes->seconds
    for num in split
        {
            // check for hour
            if res.0 == true
                {
                    // math op
                    res.1 += num.parse::<i32>().unwrap() * 60;
                    res.0 = false;
                }

            else
                {
                    res.1 += num.parse::<i32>().unwrap();
                }
                
        }

    // result
    println!("{}", res.1);
}



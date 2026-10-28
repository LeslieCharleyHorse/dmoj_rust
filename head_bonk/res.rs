use std::str::SplitWhitespace;

fn main()
{
    // input lib
    use std::io;

    // SplitWhitepsace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = String::new();

    // var to hold split vals
    let mut split_input: SplitWhitespace = "".split_whitespace();

    // var to hold res
    let mut res: i32 = 0;


    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");
//lesliecharleyhorse
    input.clear();

    // read vals
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // assign split vals into split_input var
    split_input = input.split_whitespace();


    // loop thru vals 
    for num in split_input
        {
            // check for above 0 add to res
            if num.trim().parse::<i32>().expect("Readline = Failed") > 0
                {res+=1;}
        }

    println!("{res}");

}

            
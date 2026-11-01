fn main()
{

    // input lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = "".to_string();

    // var to hold split vals
    let mut split: SplitWhitespace = "".split_whitespace();

    // var to hold res
    let mut res: String = String::new();

    // var to hold grow size
    let mut grow: i32 = 0;


    // read number count
    io::stdin().read_line(&mut input).unwrap();

    // clear input
    input.clear();


    // read grow size ,assign val to input var
    io::stdin().read_line(&mut input).unwrap();

    // assign converted int val of input to grow var
    grow = input.trim().parse::<i32>().unwrap();

    // double grow
    grow *= 2;

    // clear input
    input.clear();
    //lesliecharleyhorse


    // read numbers
    io::stdin().read_line(&mut input).unwrap();


    // split input vals, assign vals to split var
    split = input.split_whitespace();


    // loop thru split string vals
    for num in split
        {
            // add num + grow to res String 
            res.push_str(&(num.parse::<i32>().unwrap() + grow).to_string());
            // add space
            res.push_str(" ");
            
        }

    // pop last space end of string
    res.pop();
    
    // result
    println!("{}", res);

}


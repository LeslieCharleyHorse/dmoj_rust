fn main()
{
    // input lib
    use std::io;

    // Split lib
    use std::str::Split;


    //lesliecharleyhorse

    // var to hold input
    let mut input: String = "".to_string();

    // var to hold split input 
    let mut split_input: Split<'_, char> = "".to_string().split(' ');

    // var to hold res
    let mut res: String = String::new();


    // read input
    io::stdin().read_line(&mut input).expect("readline = failed");

    // split input by - , assign vals to split_input var
    split_input = input.split('-');

    // loop thru split names
    for name in split_input
        {
            // add first letter to res var
            res.push_str(&name[0..1]);
        }

    // result
    println!("{res}");
}



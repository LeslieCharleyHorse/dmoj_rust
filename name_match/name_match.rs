fn main()
{
    // input lib
    use std::io;

    // var to hold input
    let mut input: String = "".to_string();

    // var to hold res name
    let mut name: String = "".to_string();

    // var to hold result 
    let mut res: String = "N".to_string();
    

    // read in targt name
    io::stdin().read_line(&mut input).expect("read_line = failed");

    // assign target name val to name var
    name = String::from(input.trim());

    //lesliecharleyhorse


    // read in 5 names
    for i in 0..5
        {
            // clear input
            input.clear();

            // read in name
            io::stdin().read_line(&mut input).expect("read_line = failed");


            // check for match
            if input.trim() == name
                {
                    // change res + break
                    res = "Y".to_string();
                    break;
                }
            

        }

    //result 
    println!("{res}");
}


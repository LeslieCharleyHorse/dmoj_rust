fn main()
{
    // input lib
    use std::io;
    
    
    // var to hold input
    let mut input: String = String::new();
    
    // read input
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    // check substring
    if input.trim().contains("demello")
        {
            println!("liar");
        }
        
    else
        {
            println!("what are we going to do?");
        }
        
    //lesliecharleyhorse
}
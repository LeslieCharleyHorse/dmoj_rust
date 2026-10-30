fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    
    // tuple to hold (count, res)
    let mut res: (i32, i32) = (0, 0);
    
    
    // read count input
    io::stdin().read_line(&mut input).unwrap();
    
    // assign int val of count
    res.0 = input.trim().parse::<i32>().unwrap();
    //lesliecharleyhorse
    
    
    // loop from 0 to count
    for i in 0..res.0
        {
            // clear input
            input.clear();
            
            // read input num
            io::stdin().read_line(&mut input).unwrap();
            
            // add int val of input to result
            res.1 += input.trim().parse::<i32>().unwrap();
            
            
            // print result
            println!("{}", res.1);
            
        }
}

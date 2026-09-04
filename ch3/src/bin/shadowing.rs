fn main()
{
    //shadowing
    let y=0;
    let y=y+2;
    {
        let y=y*22;
        println!("The value of the inner scope of y is {y}")
    }
    println!("The value of the outer scropt of y is {y}")
}
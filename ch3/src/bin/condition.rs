fn main()
{
    let number=3;
    if number==3{
        println!("it is a number 3")
    }
    else{
        println!("not a number 3")
    }
}
//multiple condition
fn main2()
{
    let number=3;
    if number==3{
        println!("it is a number 3")
    }
    else if(number==0)
    {
        println!("number is zero")
    }
    else{
        println!("not a number 3")
    }
    
}
//using if in let statement
fn main3()
{
    let condition=true;
    let number= if condition{3} else {6};
    print!("the value of number is: {number}");

}
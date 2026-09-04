fn main()
{
    let x= call(3);
    println!("the value returned is {x}");
}
fn call(x:i32)-> i32
{
    x+1
}
fn main()
{
    println!("this is the main func");
    another_func(2,8);
}
fn another_func(a:i16,b:i16)
{
    let sum=a+b;
    println!("this is the number:{sum}")
}
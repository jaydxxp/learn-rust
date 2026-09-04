fn main()
{
    let tup=(1,3.5,62);
    let (x,y,z)=tup; //this is called destructuring where we do access the element by declaring other
    let num=tup.1; //we can access single element by tuple_name.element_index
    print!("{num}");
}
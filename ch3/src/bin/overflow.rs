fn main() {
    // integer overflow concept where the number get's wrap around to the range number as u8 
    // range is 256 so it gets minus from it and gives 8
    let mut num: u8 = 255;

    num = num + 9;

    println!("{}", num);
}
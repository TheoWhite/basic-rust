fn main()
{
    let x = 10;
    let y = "This is a string";
    /*
    Rust will not directly work with two different types
    For example {int} / {float} so
        1. Convert to {int} (as u32)
                or
        2. Convert to {float} (as f64)
        */
    let float_conversion = (150.0 * 3.1) / x as f64;
    let int_conversion = ((150.0 as u32) * (3.1 as u32)) / x as f64;

    println!("x = {}",x);
    println!("{}",y);
    println!("{}",float_conversion);
}

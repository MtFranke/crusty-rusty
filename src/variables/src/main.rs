fn main() {
    //const PI_NUMBER: f32 = 3.14;
    let x = 5;
    println!("{}", x);
    //shadowed
    let x = 6;
    println!("{}", x);

    {
        let x = x * 2;
        println!("{}", x);
    }

    println!("{}", x);

    //let mut spaces = "    ";
    //let spaces = spaces.len();
    //println!("{}", spaces);

    //overflow error
    //let mut val: u8 = 255;
    //val += 1;

    let tup = ("a", 'a', 2.11);
    let (z, x, y) = tup;
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let a = [1, 2, 3, 4];
}

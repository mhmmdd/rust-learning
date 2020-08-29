pub fn run() {
    let x = 2;

    // Input parameters are passed inside | | and expression body is wrapped within { }
    let square = |i: i32| -> i32 {
        i * i
    };

    let sum = |a:i32, b:i32| {
        a+b
    };

    println!("{}", square(2));
    println!("{}", sum(2, 3));
}
pub fn sample() {
    // Creating a tuple
    let tup = (500, 6.4, 1);
    // Destructuring a tuple
    let (x, y, z) = tup;
    
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Creating an array
    let a = [1, 2, 3, 4, 5];

    // Use ':?' to print the array in format strings
    println!("{:?}", a);
}

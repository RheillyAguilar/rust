


fn main() {

    // Fix the error below with least amount of modification to the code
    // let x: i32 = 5; // Uninitialized but used, ERROR !
    // let _y: i32; // Uninitialized but also unused, only a Warning !

    // assert_eq!(x, 5);

    // Fill the blanks in the code to make it compile
    // let  mut x: i32 = 1;
    // x = x + 2; 
    
    // assert_eq!(x, 3);

    // let x: i32 = 10;
    // let y: i32 = 5;
    // {
    //     println!("The value of x is {} and the value of y is {}", x,y);
    // }
    // println!("The value of x is {} and the value of y is {}", x,y)

    // define_x();

    // let x: i32 = 5;
    // {
    //     let x = 12;
    //     assert_eq!(x, 12);
    // }
    // assert_eq!(x, 5);

    // let x = 42;
    // println!("{}", x); // Prints "42".
    // let mut x: i32 = 1;
    // x = 7;
    // let mut x = x; 
    // x = x + 3;


    // let y: i32 = 4;
    // let y = "I can also be bound to text!"; 
    // let (mut x, y) = (1, 2);
    // x += 2;
    // assert_eq!(x, 3);
    // assert_eq!(y, 2);


    // let (x,y);
    // (x, ..) = (3, 4);
    // [.., y] = [1, 2];
    // assert_eq!([x,y], [3, 2]);


    // let x: i32 = 5;
    // let mut y = 5;
    // y = x;
    // let z: i32 = 10; // Type of z ? 



    // let x: u32 = 5;
    // assert_eq!("u32".to_string(), type_of(&x));
    // }

    // fn type_of<T>(_: &T) -> String {    
    // format!("{}", std::any::type_name::<T>())

    // let x: f64 = 1_000.000_1; // ?   
    // let y: f32 = 0.12; // f32
    // let z: f64 = 0.01_f64; // f64
    // assert_eq!(type_of(&x), "f64".to_string());
    // }
    // fn type_of<T>(_: &T) -> String {
    // format!("{}", std::any::type_name::<T>())

    assert!(0.1 as f32+0.2 as f32==0.3);
    println!("Success!");


}

// fn define_x() {
//     let x: &str = "Hello";
//     println!("{} World", x)
// }
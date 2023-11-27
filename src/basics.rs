// use std::cmp::min;                          //crate(library)::module::function - use imports this
//
// fn variables_and_pairs() {
//     let x;
//     x = 42;
//     let y = 42;
//
//     let _ = 42;                             // _ allows us to throw away value without setting off compiler
//
//     let pair = (x,y);
//     print!("{}", pair.0);
//     print!("{}", pair.1)
// }
//
// fn return_type() -> i32 {                   //defines the return type of the function
//     return 4;                               //can omit the "return" here, last line always returns
// }
//
// fn dice_roll(num: bool) -> i32 {
//     match num {                             //match is an expression
//         true => 6,
//         _ => 0                              //_ works as a catch all
//     }
// }
//
// fn type_functions() {
//     let x = "amos".len();
//     let x = str::len("amos");   //two ways of doing the same thing
// }
//
// fn structs() {
//     struct Number {                         //structs are declared
//         odd: bool,                          //variable bindings are immutable, cannot be reassinged or changed
//         value: i32                          //structs can also be generic
//     }
//
//     let x = Number {odd: false, value: 0 }; //structs can be initialised using literals
//     let mut y = Number {odd: false, value: 2};//
//
//     impl Number {                           //impl allow adding extra functionality to the struct
//         fn is_positive(self) -> bool {
//             self.value > 0
//         }
//     }
//
//     print!("{}", x.is_positive())
// }
//
// fn generic_fn<T>(arg: T) {
//     //generic function type T
// }
//
// fn vectors() {
//     let mut v1 = Vec::new();
//     v1.push(1);                         //vec is generic and changes based to assigned types
//
//     let v2 = vec![1,2,3]   ;        //vec literals achieved with macros
// }
//
// fn error_handling() {
//     let melon = &[240,159,141,137]; //error handling using match
//     match str::from_utf8(melon) {
//         Ok(s) => println!("{}", s),
//         Err(e) => panic!(e),
//     }
// }
//
// fn bubble_up_error() -> Result<_, _> {
//     let melon = &[240,159,141,137]; //error handling using match
//     match str::from_utf8(melon) {
//         Ok(s) => println!("{}", s),
//         Err(e) => return Err(e),
//     }
//     Ok(())
// }
//
// fn iterators() {
//     let natural_numbers = 1..; //lazyily generates numbers 1 to infinity as required
//     let x = (3..6).contains(&100);               //called a range
//     for i in x {
//         println!("{}", i)
//     }
//
// }
//
//
//

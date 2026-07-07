// fn main() {
//     println!("Hello, world!");
// }



//write a func is_even that takes a number as an input and returns true if it is evven
// fn main(){
//     println!("{}", is_even(20));
// }
// fn is_even(num: i32) -> bool {
//     if num%2==0 {
//         return true;
//     }
//     return false;
// }



//write a func fib that finds the fibonacci of a num it takes as input
// fn main(){
//     for i in 0..=10 {
//         println!("fib({}) = {}", i, fib(i));
//     }
// }
// fn fib(num: i32) -> i32{
//     let mut first =0;
//     let mut second=1;
//     if num==0 {
//         return first;
//     }
//     if num==1{
//         return 1;
//     }
//     for _ in 2..=num {
//         let temp=second;
//         second=second+first;
//         first=temp;
//     }
//     return second;
// }



//write a func get_string_length that takes a string as an input and returns its length
fn get_string_length(s:&str)-> usize{
    s.chars().count()
}
fn main(){
    let my_string =String::from("Hello, world!");
    let length=get_string_length(&my_string);
    println!("The number of chars in the string is: {}",length);
}
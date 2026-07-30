// fn main(){
//     let s=String::from("mohinibairagi");
//     println!("{}", is_longer_than(&s, 10));
// }

// fn is_longer_than(s: &str, num: usize) -> bool{
//     if s.len()>num{
//         return true;
//     } else{
//         return false;
//     }
// }





//structs

// #[derive(Debug)]
// struct User{
//     name: String,
//     age:u32,
//     is_admin:bool
// }

// fn main() {
//     let user1 = User {
//         name: String::from("Mohin"),
//         age: 21,
//         is_admin:true
//     };

//     let user2 = User {
//         name: String::from("xyz"),
//         age: 21,
//         is_admin:false
//     };

//     let user3 = User {
//         name: String::from("abc"),
//         age: 21,
//         is_admin:false
//     };

//     println!("{:?}", user1);
//     println!("{:?}", user2);
//     println!("{:?}", user3);
// }



//if u want ot print many users in structs
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: u32,
//     is_admin: bool,
// }

// fn main() {
//     let users = vec![
//         User {
//             name: String::from("Mohin"),
//             age: 21,
//             is_admin: true,
//         },
//         User {
//             name: String::from("xyz"),
//             age: 21,
//             is_admin: false,
//         },
//         User {
//             name: String::from("abc"),
//             age: 21,
//             is_admin: false,
//         },
//     ];

//     for user in users {
//         println!("{:#?}", user);
//     }
// }





//impl in structs
//struct=what the thing is
//impl=what the thing can do

//here new() is an associated fucntion used to create a User
//is_adukt() is a method bacause it uses &self

//self takes ownership, &self borrows, &mut self allows changing the strut

// struct User{
//     name:String,
//     age:u32
// }

// impl User{
//     fn new(name: String, age:u32) -> Self{
//         Self{name,age}
//     }

//     fn is_adult(&self) -> bool{
//         self.age>=18
//     }
// }
// fn main() {
//     let user = User::new(String::from("Mohin"), 17);

//     println!("{}", user.is_adult());
// }

//main purpose of impl is to attach methods and related fucntions to a struct or enum
//Self is the type itself while self is the instance of that type 

//It’s not that new is required. It’s just the common convention for creating a value. You can name it anything, but new is the standard Rust style.








//static and non-static functions in impl struct
//static function=no self paramenter; non-static : has self, &self or &mut self as parameter
// struct User{
//     name: String,
//     age: u32,
// }

// impl User{
//     fn new(name: String, age: u32) -> Self{
//         Self{name,age}
//     }

//     fn who_am_i() -> String{
//         String::from("I am the user struct")
//     }

//     fn is_allowed_to_vote(&self, legal_age:u32) -> bool{
//         self.age>=legal_age
//     }
// }

// fn main() {
//     let user1 = User::new(String::from("mohini"), 18);
//     let user2 = User::new(String::from("Raman"), 13);

//     println!("{}", User::who_am_i());

//     println!("{} can vote: {}", user1.name, user1.is_allowed_to_vote(18));
//     println!("{} can vote: {}", user2.name, user2.is_allowed_to_vote(18));
// }

// User::something() = function belongs to the struct type itself.
// user1.something() = function belongs to one actual value.









//enums in rust
//use an enum when a vlue can be one of several forms: a direction, a status, a msg type, etc
//example; enum Direction {Up, Down, Left, Right} : this means a direction val must be exactly oen of those 4 variants 

//enums can also carry data inside variants
//ex: enum Message { Quit, Move{x:i32, y:i32}, Write(String), ChangeColor(u8,u8,u8)}

// enum Shape{
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64)
// }

// fn calculate_area(shape: Shape) -> f64 {
//     if let Shape::Circle(radius)=shape{
//         return radius*radius*3.14;
//     }

//     if let Shape::Square(side)=shape{
//         return side*side;
//     }

//     if let Shape::Rectangle(width, height) =shape{
//         return width*height;
//     }

//     return 0.0;
// }

// fn main(){
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     let result1 = calculate_area(circle);
//     let result2 = calculate_area(square);
//     let result3 = calculate_area(rectangle);

//     println!("{}", result1);
//     println!("{}", result2);
//     println!("{}", result3);
// }
//if let only matches one case at a time.







//enum with pattern matching i.e. match
//Here, the condition is the shape of the data itself. Rust uses that shape to safely pull out the values without unsafe casting.
// enum Shape{
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64,f64)
// }

// fn calculate_area(shape: Shape) -> f64{
//     match shape{
//         Shape::Circle(radius)=> radius*radius*3.14,
//         Shape::Square(side)=> side*side,
//         Shape::Rectangle(width, height)=> width*height,
//     }
// }

// fn main(){
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     let result1 = calculate_area(circle);
//     let result2 = calculate_area(square);
//     let result3 = calculate_area(rectangle);

//     println!("Circle area: {}", result1);
//     println!("Square area: {}", result2);
//     println!("Rectangle area: {}", result3);
// }








//error handling with result enum
//in rust,the standard way to handle recoverable errors is to return s Result<T,E> where, Ok(T) is success and Err(E) is the custom error, this lets u use ? and pattern matching cleanly

// #[derive(Debug)]

// enum ParseAgeError{
//     EmptyInput,
//     InvalidNumber,
//     TooSmall
// }

// fn parse_age(input: &str) -> Result<u32, ParseAgeError>{
//     if input.trim().is_empty() {
//         return Err(ParseAgeError::EmptyInput)
//     }
//     let age: u32 = input
//         .trim()
//         .parse()                                         //tries to convert the text into a u32
//         .map_err(|_| ParseAgeError::InvalidNumber)?;           //if .parse() fails, rust normally gives a parse error, map_err() converts that error into ur custom error :ParseAgeError::InvalidNumber
//         //the ? above means if there is an error, return it immediately 

//     if age == 0 {
//         return Err(ParseAgeError::TooSmall);
//     }

//     Ok(age)
// }
// fn main() {
//     match parse_age("18") {
//         Ok(age) => println!("Age: {}", age),
//         Err(err) => println!("Error: {:?}", err),
//     }
// }









//option enum: Option<T>
//means a val is either present or absent
//in rust, this is written as Some(value) or None

// fn find_score(name: &str) -> Option<u32>{
//     match name{
//         "Aman" => Some(90),
//         "Rohan" => Some(75),
//         _ => None
//     }
// }

// fn main(){
//     let student1="Aman";
//     let student2="Zara";

//     match find_score(student1) {
//         Some(score) => println!("{} got {}", student1, score),
//         None => println!("{} not found", student1),
//     }

//     match find_score(student2) {
//         Some(score) => println!("{} got {}", student2, score),
//         None => println!("{} not found", student2),
//     }
// }










//Option<String>
// #[derive(Debug)]
// struct User{
//     name: String,
//     middle_name: Option<String>
// }

// impl User{
//     fn new(name:String, middle_name:Option<String>) ->Self{
//         Self{name, middle_name}
//     }

//     fn print_name(&self){
//         match &self.middle_name{
//             Some(middle)=>println!("{} {}", self.name,middle),
//             None=>println!("{}",self.name)
//         }
//     }
// }

// fn main(){
//     let user1=User::new(String::from("mohini"), Some(String::from("bairagi")));
//     let user2 = User::new(String::from("Raman"), None);

//     user1.print_name();
//     user2.print_name();

//     println!("{:?}", user1);
//     println!("{:?}", user2);
// }










//generics and trait bounds
//generics let u write one function for many types, and trait bounds let u restrict which types are allowed
//a generic type like T is a placeholder, the compiler replaces it with a real type when u call the func


//a generic func that returns the first item of a vector, regardless of whether th vector contains integers, strings or floats
// pub fn main(){
//     let v=vec![1,2,3];
//     let v2 = vec![String::from("mohini"), String::from("bairagi")];
//     let v3=vec![1.0,2.0,3.0];
//     println!("{}", first_element(v).unwrap());
//     println!("{}", first_element(v2).unwrap());        //unwrap() extracts the value inside Some()
//     println!("{}", first_element(v3).unwrap());
// }
// fn first_element<T>(v: Vec<T>) -> Option<T>{
//     return v.into_iter().nth(0);
//     //This consumes the vector and turns it into an iterator over its values.
//     //.nth(0) asks for the element at index 0, which is the first element. If the vector is empty, it returns None
// }


//below code shows gnerics, trait bounds and boerowing vs ownership all together
//thing to notice: first_element consumes its vector while does_exist only borrows it
// pub fn main(){
//     let v = vec![1, 2, 3];
//     let v2 = vec![String::from("Harkirat"), String::from("Singh")];
//     let v3 = vec![1.0, 2.0, 3.0];

//     println!("{}", first_element(v.clone()).unwrap());                //v.clone() vreates a copy of v so that it can be used later in does_exist
//     println!("{}", first_element(v2).unwrap());                        //takes ownership of v2 and gets the first elemet and prints it
//     println!("{}", first_element(v3).unwrap());

//     println!("{}", does_exist(v, 1));                                 //checks whether 1 is inside the original vector v
// }
// fn first_element<T>(v: Vec<T>) -> Option<T> {
//     v.into_iter().nth(0)
// }

// fn does_exist<T: std::cmp::PartialEq>(v: Vec<T>, element: T) -> bool {
//     let mut iter = v.iter();
//     while let Some(value) = iter.next() {
//         if *value == element {                                         //value is the ref so *value is the actual value, rust comapres it with element
//             return true;
//         }
//     }
//     false
// }
//PartialEq is req cuz the func uses ==









//generics over structs
//means u define a struct with type placeholders like T ur U so the same struct can hold different kinds of data.
//in rust, the generic type gets fixed when you create the value
// struct Rect{
//     width: f64,
//     height: f64,
// }
// impl Rect{
//     fn area(&self) -> f64{
//         self.width*self.height
//     }
// }
// fn main(){
//     let rect1=Rect{width: 10.0, height: 5.0};
//     let rect2=Rect{width: 3.0, height: 4.0};

//     println!("Area of rect1: {}", rect1.area());
//     println!("Area of rect2: {}", rect2.area());
// }



//adding a trait bound to it
// use std::ops::Mul;
// struct Rect<T>{
//     width: T,
//     height: T,

// }
// impl<T> Rect<T> where T: std::ops::Mul<Output=T> + Copy{                 //+copy is there bcz ur method needs ot copy the fields out of &self in order to multiply them
//     fn area(&self) -> T{
//         self.width*self.height
//     }
// }
// fn main(){
//     let rect1=Rect{width: 10.0, height: 5.0};
//     let rect2=Rect{width: 3.0, height: 4.0};    
//     println!("Area of rect1: {}", rect1.area());
//     println!("Area of rect2: {}", rect2.area());
// }

//in the non-generic version above this one, f64 already implements both copy and Mul<Outputf64> so u dont hv to write any bounds, rust knows 
//impl<T> introduces a type vairable T
//Rect<T> means use that ttype variable T in the struct definition








//generics over enums
enum Container<T>{
    Some(T),
    Empty
}
impl<T> Container<T>{
    fn is_empty(&self) -> bool{
        match self{
            Container::Some(_)=>false,
            Container::Empty=>true
        }
    }
}
fn main(){
    let c1: Container<i32> = Container::Some(10);
    let c2: Container<String> = Container::Empty;
    println!("Is c1 empty? {}", c1.is_empty());
    println!("Is c2 empty? {}", c2.is_empty()); 
}
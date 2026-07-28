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

enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shape) -> f64 {
    if let Shape::Circle(radius)=shape{
        return radius*radius*3.14;
    }

    if let Shape::Square(side)=shape{
        return side*side;
    }

    if let Shape::Rectangle(width, height) =shape{
        return width*height;
    }

    return 0.0;
}

fn main(){
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    let result1 = calculate_area(circle);
    let result2 = calculate_area(square);
    let result3 = calculate_area(rectangle);

    println!("{}", result1);
    println!("{}", result2);
    println!("{}", result3);
}
//if let only matches one case at a time.
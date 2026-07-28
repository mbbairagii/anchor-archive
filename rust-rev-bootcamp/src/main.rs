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
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    is_admin: bool,
}

fn main() {
    let users = vec![
        User {
            name: String::from("Mohin"),
            age: 21,
            is_admin: true,
        },
        User {
            name: String::from("xyz"),
            age: 21,
            is_admin: false,
        },
        User {
            name: String::from("abc"),
            age: 21,
            is_admin: false,
        },
    ];

    for user in users {
        println!("{:#?}", user);
    }
}
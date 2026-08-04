//serde is a rust lib for serialization and deserialization, it turns data into fromats like json and turns json back into rust data
//u usually use it with #[derive(Serialize, Deserialize)]

//serialize=convert rust data into a string or bytes format
//deserialize=convert that string or bytes back into rust types
//so serde gives u the traits and derive macros so u dont write the conversion code by hand

//commonly used in reading/writing json files, apis, config files, n/w msgs, binary encoding with crates like bincode

//example:

// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     id: u64,
//     name: String,
// }

// fn main() {
//     let user = User {
//         id: 1,
//         name: String::from("Alice"),
//     };

//     let json = serde_json::to_string(&user).unwrap();
//     println!("{}", json);

//     let back: User = serde_json::from_str(&json).unwrap();
//     println!("{:?}", back);
// }










//assigmnet-1
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]

// struct User{
//     id: u64,
//     name: String,
// }

// fn main(){
//     let user =User{
//         id: 1,
//         name: String::from("mohini"),
//     };

//     let yaml=serde_yaml::to_string(&user).unwrap();
//     println!("{}",yaml); //uses dispaly

//     let back: User = serde_yaml::from_str(&yaml).unwrap();
//     println!("{:?}", back);  //uses debug
// }









//assignment-2
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all= "camelCase")]

struct User {
    id: u64,
    name: String,
    address : String,
    pin_code : String,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("mohini"),
        address: String:: from("delhi"),
        pin_code: String::from("123456"),
    };

    let yaml = serde_yaml::to_string(&user).unwrap();
    println!("{}", yaml);

    let back: User = serde_yaml::from_str(&yaml).unwrap();
    println!("{:?}", back);
}


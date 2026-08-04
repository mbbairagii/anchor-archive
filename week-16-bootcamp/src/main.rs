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


















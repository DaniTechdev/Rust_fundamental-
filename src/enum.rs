// enum Number{
//     Zero, //0
//     One,//1
//     Two,//2
// }

// enum Number1{
//     Zero = 0,
//     One,
//     Two,
// }

// //C-like enum
// enum Number2{
//     Zero = 0,
//     One , //6
//     Two ,//7
// }

// #[derive(Debug)]

// enum Message{
//     Quit,
//     Move{x:i32, y:i32},
//     ChangeColor(i32,i32,i32),
//     Write(String),
    
// }


// fn show_message(message:Message){
// println!("{:?}",message);
// }

fn main(){

    //fill in the blank to make the `println` work
    //Also add some code to prevent the `panic` from running

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let  n = six {
        println!("{}",n);

        println!("Success");
    }

    panic!("Never let this run!");

    // //FILL  in the blank and fix the error
    // let msgs: [Message; 3] = [
    //     Message::Quit,
    //     Message::Move{x:1,y:3},
    //     Message::ChangeColor(255, 255, 0)
    // ];

    // for msg in msgs {
    //     show_message(msg)
    // }

        // let msg: Message = Message::Move{x:1,y:2};

        // if let Message::Move{x:a, y:b} = msg {
        //     assert_eq!(a,b);
        // } else{
        //     panic!("Never Let this Run");
        // }

        // println!("Success");




//     let msg1:Message = Message::Move{x:1,y:2}; //Insantiate with x=1, y=2
 
//    let msg2:Message = Message::Write(String::from("Hello, world")); //Intantiate with "hello, world",


//     println!("success");
//     //enum

    // enum IpAAddrr {
    //     V4(String),
    //     V6(String),
    // };

    // let home = IpAAddrr::V4(String::from("127.0.0.1"));//old formaat of IP address

    // let loopback = IpAAddrr::V6(String::from("::1")); //new format of IP address


    //An enum variamt can be converted to an integer by `as`
    // assert_eq!(Number::One as u8, Number1::One as u8);

    // assert_eq!(Number1::One as u8,Number2::One as u8);

    // println!("{}", Number::One as u8);
    // println!("Success");
}
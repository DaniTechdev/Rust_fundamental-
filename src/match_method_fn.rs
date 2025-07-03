

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents (coin:Coin) -> u8{
//     match Coin {
//         Coin::Penny => 1,
//         Coin::Nickel=> 5,
//         Coin::Dime=> 10,
//         Coin::Quarter=> 25,
//     }
// }

// enum Direction {
//     East,
//     West,
//     North,
//     South
// }
#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32, y:i32},
    ChangeColor(i32,i32,i32),
    notExcuted
}


fn show_message (msg:Message)-> Message{
    match msg {
        Message::Move { x:a, y:b } => {//match Messgae::Move
            assert_eq!(a,1);
            assert_eq!(b,3);

            // println!("{:?}", Message::Move{x:a,y:b});

            Message::Move { x: 1, y: 3 }

        },

        Message::ChangeColor(r, g, b)=> {
            assert_eq!(g,255);
            assert_eq!(b,0);

            // println!("{:?}", Message::ChangeColor(r, g, b));

            Message::ChangeColor(r, g, b)

        },

        _ =>  Message::Quit
        
    };

    Message::notExcuted
}

fn main(){
    //Pattern Maatch
    //Examples of fmatch

    let msgs: [Message; 4] = [
        Message::Quit,
        Message::Move{x:1,y:3},
        Message::ChangeColor(255, 255, 0),
        Message::notExcuted

    ];


    for msg in msgs {
     let messageData: Message =  show_message(msg);

     println!("{:?}", messageData)
    }

    println!("Success");

    // println!("{:?}", msgs)
    

    // let config_max = Some(3u8);

    // match config_max {
    //     Some(max)=> println!("The maximum is configured to be {}", max),
    //      __ => (),

    // }

    //or preferable using if let

    // if let Some(max) = config_max{
    //     println!("The maximum is configured to be {}", max);
    // }



    // let dire: Direction = Direction::South;

    // match dire {
    //     Direction::East => println!("East"),
    //     Direction::South | Direction::North => println!("south or North"),
    //     _ => println!("West"),
       
    // }


    // let boolean: bool = true;

    // //Fill the blank with a match expresion

    
    // //bolean = treu => binary = 1;
    // //boolean = false => binary = 0

    // let binary: u8 = match boolean{
    //     true => 1,
    //     false => 0
    // };

    // assert_eq!(binary,1);
    // println!("Success");



    //Using match to get the data an enum variant holds


    

}
// /Fill in the  blanks

use std::collections::HashMap;

fn main (){
    //Type inference lets us omit an explicit type signature(which 
    //woould be `HashMap<*str,u8>` in this example)
    let mut player_stats = HashMap::new();



    //insert a key only if it does'nt already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"],100);

    //Insert a key using function that provides a new value only if it
    //does not already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    //Ensure a value is the entry by inserting the default if empty, aand return
    //mutable reference to the value in the entry

    let health: &mut u8 = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -=50;
    assert_eq!(*health,50);

    println!("Success!");

}

fn random_stat_buff()-> u8{
    //could aactually return some randome value here -> let's just return
    //some fixed vaalue for now
    42
}
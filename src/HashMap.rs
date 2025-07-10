//Fill in the blanks and Fix errors
use std::collections::HashMap;

fn main(){
    let mut scores:HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("John", 58);

    //get method get() returns an Option<&V>
    let score: Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel"){
        //Indexing returns a value V
        let score: i32 =  scores["Daniel"];
        assert_eq!(score, 95);

        scores.remove("Daniel");
        assert!(!scores.contains_key("Daniel"));
        // println!("Daniel's score removed");

    }

        assert_eq!(scores.len(), 3);
    //Iterate over the HashMap
    for (name,score) in scores{
        println!("The score of {} is {}", name, score);
        
    }

}
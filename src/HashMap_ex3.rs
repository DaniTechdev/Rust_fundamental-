use std::collections::HashMap;

fn main(){
    let mut map = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);

    //Indeed , the capacity of HashMap is not , so we can't compare equality

    assert!(map.capacity() >= 100, "HashMap capacity should be at least 100");

    //Shrink the capacity of the map with a lower limit. It will drop
    //down no loweer than the supplied limit while maintaining the internal rules
    //and possible leaving some space in accordance with the resize policy.

    map.shrink_to(50);

    //Shrink the capacity of the map as much as possible.It will drop
    //down as much as possible while maintaining the internal rules
    //and possible leaving some space in accordance with the resize policy.

    map.shrink_to_fit();
    assert!(map.capacity() >= 2);

    println!("Success");
}
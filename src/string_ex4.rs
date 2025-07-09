//Fill in the blank and Fix errors

fn main(){
    let s:String = String::from("hello,##");
    let slice1 = &s[..1]; //tips: `h` only takes 1byte in UTF8 format
    assert_eq!(slice1, 'h'.to_string());

    let slice2: &str = &s[7..8]; //Tips `#` takes 3 bytes in UTF8 format
    assert_eq!(slice2, "#");

    //Iterate throught the chars in s

    for (i,c) in s.chars().enumerate() { //enumerate() returns (index, char) tuple
      if i == 7{
        assert_eq!(c, '#');
      };
    }

    println!("success");
}
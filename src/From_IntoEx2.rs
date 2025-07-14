//Example converting a string slice into a string

fn main(){
    let my_str = "hello";

    //three conversion below all depends on the fact: String implements From<&str>
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();

    //Explicity type annotation is required here

    let string3:String = my_str.into();


}






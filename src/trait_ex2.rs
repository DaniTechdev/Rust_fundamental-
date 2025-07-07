//`centimters`, a tuple struct that can be compared

#[derive(PartialEq,PartialOrd)]
struct  Centimeters(f64);

//`Inches , a tuple struct tht can be printed`
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters{
        let &Inches(inches) = self; //Inches(12) will be destructure to self. self was dereferenced because we want to take the value not the reference

        Centimeters(inches as f64 * 2.54)
    }
}


//Add some attributes to make the code work!

//Don't modify other code!
#[derive(Debug,PartialEq,PartialOrd)]
struct  Seconds(i32);

fn main(){
 let _one_second:Seconds = Seconds(1);

println!("One second looks like: {:?}", _one_second);

let _this_is_true = (_one_second == _one_second);
let _this_is_true = (_one_second > _one_second);

let foot: Inches = Inches(12);

println!("One foot equals {:?}", foot);

let meter: Centimeters = Centimeters(100.0);

let cmp: &str = 
    if foot.to_centimeters() < meter {
        "smaller"
    }else{
        "bigger"
    };

    print!("One foot is {} than one meter", cmp)
}


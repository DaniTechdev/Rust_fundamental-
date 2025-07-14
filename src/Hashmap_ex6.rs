//Conversion by as

//fix the erros and fill in the blank
//Don't remove any code

fn main(){
    let decimal = 97.123_f32;

    let interger:u8 = decimal as u8; //the fractional part is be removed and the value is converted to an integer type 97.


    let c1:char = decimal as u8 as char; //the value is first converted to an integer type and then to a char type //a


    let c2:char = interger as char;

    assert_eq(interger +1, 'b' as u8);//the value of 'b' is 98, so interger is 97

    println("success");
}
// TryFrom and TryInto are included in `std::prelude` , so there is no need to introduce
// use std::convert::TryInto;

fn main(){

    let n:i16 = 256;

    //Into trait has a method `into`,
    //hence TryInto has a method, tryInto

    let n: u8 = match n.try_into(){
        Ok(n) => n,
        Err(e)=> {
            println!("there is an error when comverting: {:?}, but we catch it", e.to_string() );
            0
        }
    };

      assert_eq!(n,0);

        println!("Success");
}
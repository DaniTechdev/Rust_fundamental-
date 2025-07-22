//Make aa constant with `'static` lifetime
static NUM:i32 = 18;
// const NUM:i32 = 18;

//Return a reference to `NUM` where its `'static`
//lifetime is coerced to that of the input argument

fn coerc_static <'a> (_:&'a i32) -> &'a i32{
    &NUM
}

fn main(){
    {
        //Make an integer to use for `coerce_static`
        let lifetime_num: i32 = 9;

        //Coerce `NUM` to lifetime of `lifetime_num`
        let coerced_static: &i32 = coerc_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible",NUM );
}
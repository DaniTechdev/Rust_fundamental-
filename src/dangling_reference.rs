fn main(){
    //Example Dangling Reference

    let reference_to_nothing = dangle();
}

//This function below will violate the second rule which states
//that references must always be valid, once its out of scope, its not longer valid
fn dangle() -> &String {
    let s = String::from("hello"); //s will go out of scope after the function
                                            //this will lead to no pointer for the varible where it is return in the 
                                            //main function
    &s
}


//this below will give no dangle 
fn no_dangle ()-> String{
    let s = String::from("hellow");

    s
}
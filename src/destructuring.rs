fn main(){
    //destructuring a tuple
    // let  (mut  x,y) = (1,2); //tupple destructuring

    // x += 2;

    // assert_eq!(x,3);
    // assert_eq!(y,2);


        let (x,y);

        (x,..) = (3,4);
        [..,y] = [1,2]; //array

        //fill the  blank to make code work

        assert_eq!([x,y],[3,2]);

        println!("success!");

}
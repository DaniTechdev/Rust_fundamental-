
enum MyEnum{
    Foo,
    Bar
}

enum  Foo1{
    Bar(u8)
}

enum Foo{
    Bar,
    Baz,
    Qux(u32)
}

fn main(){
    //matches! looks like match but can be something different

    // let alphabets = ['a','E','Z','0','X','9','Y'];

    // //fill  the blank wih `matches` to make the code woek

    // for ab in alphabets {
    //     assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0' ..='9'))
        
    // }

    // println!("Success");
    // let mut count = 0;

    // let v:Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];  //vec is vector similarly to an array but with dynamice size. The size can shrink or expand unilke an array

    // for e in v {
    //     // if e == MyEnum::Foo {//Fill the eroor by changing only this line
    //     //     count+=1;
    //     // }
    //     if matches!(e,MyEnum::Foo){
    //         count+=1;
    //     }
    // }

    // assert_eq!(count,2);

    // println!("Success");


    //Fo some cases , when matching enums is too heavy, we ca use if let instead

    // let o = Some(7);


    // match o {
    //     Some(i)=> {
    //         println!("This is aa reality long string and `{:?}", i);

    //         println!("Success");
    //     }
    //     _ => {}
    // }
        //     if let Some(i) = o {
        //                    println!("This is aa reality long string and `{:?}", i);

        //                    println!("Success");

        //    }

            //




    // fill in the blank

    // let a: Foo = Foo::Bar(1);

    // if let Foo::Bar(i) = a{
    // println!("Foobar holds the value: {:?}", i);

    // println!("Success")
    // }



        // let a: Foo = Foo::Qux(10);

        // //Remove the codes below , using `match ` instead

        // // if let Foo::Bar  = a {
        // //     println!("match foo::bar")
        // // }else if let Foo::Baz = a {
        // //     println!("match::baz")
        // // } else{
        // //     println!("match others")
        // // }

        // match a {
        //     Foo::Bar =>{
        //         println!("match foo::Bar")
        //     },
        //     Foo::Baz=>{
        //         println!("mathc foo::Baaz")
        //     },
        //     Foo::Qux(i) =>{
        //         println!("match foo::Quz")
        //     }
        // }



//         //Shadowing
//         let age: Option<i32> = Some(30);

//         if let Some(age) = age {//create a new variable with naame as previous
//             assert_eq!(age, 30); //Some(age) = age means the age inside Some(age) is now a new variable and age Option<i32> varible value has been assigned to it in the new scope
//             }// The new varibale `age` goes out of scope here

//         match age {
//             Some(age)=> println!("age is a new variable, it's vaalue is {}", age),
//             _ => ()
//         }
// }

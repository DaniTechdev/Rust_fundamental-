//make it work 
//Don't use `_reborrow` and `_count_reborrowed`
//Don't modify `assert_eq`

fn main(){
    let mut count = 0;

    let mut inc =move || {
        count +=1;
        println!("`count`: {}", count);
    } ;

    println!("{} outer", count);


    inc();


    let _reborrow: &i32 =&count;

    inc();

    //The closure no longer needs to borrow `&mut count`. Therefore, it is 
    //possible to reborrow without an error

    let _count_reborrowed: &mut i32 = &mut count;

    assert_eq!(count,0);
}
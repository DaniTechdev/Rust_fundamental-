//Fix the errors

fn main(){
    let mut v: Vec<i32> = vec![1,2,3];

    let slice1: &[i32] = &v[..];
    //out of bound will cause a panic
    //You must use `v.len` here
    let slice2: &[i32] = &v[0..v.len()];

    assert_eq!(slice1, slice2);
    //Slices are read only
    //note: slice and &Vec are different

    let vec_ref:&mut Vec<i32> = &mut v;
    (*vec_ref).push(4);

    let slice3: &[i32] = &v[0..v.len()]; //or &v[..]
    // slice3.push(4);  Note: we can't use a slice to mutate the vector/data but to view on the date/read only
    
 


    assert_eq!(slice3, &[1,2,3,4]);
}
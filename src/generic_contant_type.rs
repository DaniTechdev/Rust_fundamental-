

//<T, const N:usize> is part of the struct type, it means Array<i32,3> and Array<i32,4> are different types


struct Array<T, const N:usize>{
    data:[T;N] //[f64;2]
}


fn main(){
    let arrays:[Array<i32,3>; 3]= [
        Array{
            data: [1,2,3],
        },
        Array{
            data: [1,2,3],
        },
        Array{
            data:[1,2,3],
        }
    ];


    let floats : [Array<f64,2>;3]=[
        Array{ data:[3.2,4.5]},
        Array{ data:[2.3,4.5]},
        Array{data:[1.1,1.0]},
    ];
}
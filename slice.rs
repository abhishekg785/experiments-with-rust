//slice : dynamicaaly sized reference to another ds , think you want
//get/pass a part of array of any another ds, instead of copy it to another array, rust allows to
//create a reference/view to access only that part of the data
//and it can be mutable or not

let a[i32, 4] = [1,2,3,4]; //parent array

let b: &[i32] = &a;  //slicing whole array
let c = &a[0..4]; //from oth to 4th (excluding)

let d = &a[..]; //slicing whole array

let e = &a[1..3]; //[2,3]

let e = &a[1..]; //2,3,4
let e = &a[..3] //1,2,3


// arrays are immutable by default and also even with mut keyword its element count cannot be
// changed

fn main() {
    let a = [1,2,3,4]; 
    let mut b = [1,2,3];

    let c: [int; 3] = [1,2,3]; // [type, no of elements]

    let d: ["my value"; 3];    // ["my value", "my value", "my value"]

    let e: [132; 0] = [];  //empty array    

    println!("{:?}", a); //[1,2,3]

    println!("{:#?}", a);
}

fn main() {
    println!("hello_world");
}

fn print_sum(a :18, b:18) {
    println!("sum is :{}", a+b);
}

//Returning
fn plus_one(a:i32) -> i32 {
    a+1 //no ; means an expression returning a+1
}

fn plus_two(a:i32) -> i32 {
    return a+2; //return a+2 but bad practise,
    //should use only conditional return , except its last expression
}

// Functions pointers , Usage as a data type
let b = plus_one;
let c = b(5);

//checking ownership
// if the resource is borrowed once, cannot be borrowed and used more than once during the prgramme exec
fn main() {
    // let mut x = 5;
    // x = 10;
    // let y = x;
    // println!("{}", x + 1 );	
    // println!("{}", y);
    let mut x = 10;
    let mut y = &mut x;  //mutable reference

    *y = 10;
    println!("{}", y);
    //println!("{}", x);  //will shoot a error as the y has borrowed the x ( mutable borrow) and the source can not be shared more than once if borrowed
}

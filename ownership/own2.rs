fn make_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);
    vec   //tranferring ownership to the caller
}

fn print_vec(vec :Vec<i32>) {
    // the vec parameter is a part of this scope so it is owned by the 'print_vec'
    for i in vec.iter() {
        println!("{}", i)
    }

    //now 'vec' is deallocated
}

fn use_vec() {
    let vec = make_vec();  //take ownership of the vector
    print_vec(vec);        // pass the ownership

    /*
    for i in vec.iter() {   //not accessible once the ownership is padded
        println!("{}", i);
    }
    */
}

fn main() {
    use_vec();
}

//just befor make_vec's scope ends , vec is moved out by returning it, it is not destroyed 
//and then the use_vec get the ownership

//and then the ownerhip gets tranferred to the print_vec bro :P


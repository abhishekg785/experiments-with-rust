fn main() {
    let a = (1, 1.5, true, 'a', "hello world!");

    let b: (i32, f64) = (1, 1.5);
    let(c,d)= b;
    let (e, _, _, _, f) = a; //indicates not interested in that item
    let g = (0,); // single element tuple
    let h = (b, (2,4), 5);

    println!("{:?}", a);
    println!("{:#?}", a);
}

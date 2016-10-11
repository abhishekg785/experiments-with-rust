fn leader() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    use(&vec);  //borrowing concept
}

fn use(vec: &Vec<i32) {

}

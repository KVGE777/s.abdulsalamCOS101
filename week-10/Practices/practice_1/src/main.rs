fn main() {

    let v = vec![101, 250, 330, 400];
    // vector 'v' owns the object in a heap

    // only a single variable owns the heap memory at any given time
    let v2 = v;

    // two pointers to the same scope or content isn't allowed
    // 'v2' assumes ownership, 'v' is discarded

    //Rust is very smart in terms of memeory access/management, so it detects a race
    //condition as two variables point to the same heap

    println!("{:?}", v);
    //'v' has been discarded and it's value transferred to 'v2'. Thus, printing 'v'
    //will result in a error
}
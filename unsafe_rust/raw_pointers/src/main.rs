fn main() {
    // define a pointer to some arbitrary location in memory
    let address = 0x01234usize;
    let r = address as *const i32;

    let mut num = 5;
    let p1 = &num as *const i32; //immutable reference
    let p2 = &mut num as *mut i32; //mutable reference
                                   // pointers can be created in safe code but not dereferenced
                                   // println!("p1 points to {}", *p1); is invalid here

    unsafe {
        println!("p1 points to {}", *p1);
        println!("p2 points to {}", *p2);
        println!("r points to {}", *r); // undefined behaviour could seg fault could give random value
    }
}

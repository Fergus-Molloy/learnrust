struct SmartPointer {
    data: String,
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data {}", self.data);
    }
}
fn main() {
    let c = SmartPointer {
        data: String::from("my stuff"),
    };
    let d = SmartPointer {
        data: String::from("other stuff"),
    };
    println!("pointers created",);
}

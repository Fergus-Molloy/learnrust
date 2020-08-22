#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>, // private so other programs can't change without updating the average
    average: f64,
}

impl AveragedCollection {
    // no new is defined as user is expected to create collection as shown in main()
    // this can lead to the average being out of sync until one of the below functions are run
    // this could be solved either using traits to ensure mutiple creation options or by forcing the user
    // to create an empty collection before adding items (this could be frustrating)
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
    let col = AveragedCollection {
        list: vec![],
        average: 0.0,
    };
    println!("Col: {:?}", col);
}

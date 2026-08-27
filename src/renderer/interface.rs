pub struct Interface {
    pub thread: std::thread::JoinHandle<()>
}

pub fn test_function() {
    for i in 1..100 {
        println!("thread hi {}", i);
    }
}

impl Interface {
    pub fn new() -> Self {
        Self{thread: std::thread::spawn(|| {test_function()})}
    }
}
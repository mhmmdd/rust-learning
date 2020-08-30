mod phrases {
    pub mod greetings {
        pub fn hello() {
            println!("Hello, world! from module.rs");

            // private function call
            super::hello();
        }
    }

    pub fn greet() {
        hello(); // Or `self::hello();`
    }

    // private function
    fn hello() {
        println!("Hello, world! from module.rs private function");
    }
}

pub fn run() {
    phrases::greetings::hello();


    // private functions are not accessible
    //phrases::hello();
    phrases::greet();
}
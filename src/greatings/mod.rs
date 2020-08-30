mod another_module;

// greatings is directory root file
// ↳ greetings/greatings
pub fn hello() { // ⭐️ The function has to be public to access from outside
    println!("Hello, world!");

    another_module::hello();
}
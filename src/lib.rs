#[uniffi::export]
fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

#[derive(uniffi::Object)]
struct Greeter {
    greeting: String
}

#[uniffi::export]
impl Greeter {
    #[uniffi::constructor]
    fn new(greeting: String) -> Self {
        Self { greeting }
    }

    fn greet(&self, name: String) -> String {
        format!("{}, {name}!", self.greeting)
    }
}

uniffi::setup_scaffolding!("lua_ffi");
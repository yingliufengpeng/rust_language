mod english {
    pub mod greetings {
        pub fn hello() -> String {
            "Hello!".to_string()
        }
    }

    pub(crate) mod farewells {
        pub fn goodbye() -> String {
            "Goodbye".to_string()
        }
    }
}

mod japanese {
    pub mod greetings {
        pub fn hello() -> String {
            "こんにちは".to_string()
        }
    }

    pub  mod farewells {

        pub fn goodbye() -> String {
            "さようなら".to_string()
        }
    }


}

pub use self::japanese::farewells;

mod tests;
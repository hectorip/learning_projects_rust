pub mod car {
    pub mod electric {
        pub fn turn_on_lights() {
            println!("🚖💡");
        }
    }
}

pub mod plane {
    pub mod mechanic {
        pub fn turn_on_engines() {
            println!("✈️");
        }
    }
}

pub fn testing_modules() {
    println!("Probando módulos en Rust!");
    crate::car::electric::turn_on_lights();
    crate::plane::mechanic::turn_on_engines();
}

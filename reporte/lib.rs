#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reporte {

    #[ink(storage)]
    pub struct Reporte {}

    impl Reporte {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn reportar(&self) {
            // Implement your message here
        }
    }
}

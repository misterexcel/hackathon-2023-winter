#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod minter {

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    // Almacena el estado del contrato
    #[ink(storage)]
    pub struct Minter {
        total_supply: Balance,
        owner: AccountId,
    }

    impl Minter {
        // Setea el estado inicial del contrato
        #[ink(constructor)]
        // Recibe un tipo "Balance", para valores en tokens
        pub fn new(Total_supply: Balance) -> Self {
            let caller = Self::env().caller();

            Self {
                total_supply: Total_supply,
                owner: caller
            }
        }
        // Transfiere valores a una cuenta destino
        #[ink(message)]
        // Recibe como parametro:
        // to: De tipo AccountId
        // value: De tipo Balance
        pub fn transfer(&mut self, to: AccountId, value: Balance) {
            let from = self.owner;
            self.env().transfer(to, value);
        }
    }

}

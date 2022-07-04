#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod real_estate_contract {
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotTheGovernanceError,
        PropertyNotApproved,
        PriceTooLess,
    }

    pub type PropertyId = u32;
    pub type Result<T> = core::result::Result<T, Error>;
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct RealEstateContract {
        governance: AccountId,
        property_owner: Mapping<PropertyId, AccountId>,
        property_name: Mapping<PropertyId, String>,
        property_price: Mapping<PropertyId, u32>,
        property_approved: Mapping<PropertyId, bool>,
        property_count: u32,
        set: bool,
    }

    impl RealEstateContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {
                Self {
                    governance: Default::default(),
                    property_owner: Default::default(),
                    property_price: Default::default(),
                    property_name: Default::default(),
                    property_approved: Default::default(),
                    property_count: 0,
                    set: false,
                };
            })
        }

        #[ink(message)]
        pub fn get_govt_id(&mut self) -> AccountId {
            if !self.set {
                self.governance = Self::env().caller();
                self.set = true;
            }
            self.governance
        }

        #[ink(message)]
        pub fn sell_property(&mut self, name: String, price: u32) -> u32 {
            self.property_approved.insert(self.property_count, &false);
            self.property_name
                .insert(self.property_count, &String::from(name));
            self.property_price.insert(self.property_count, &price);
            self.property_owner
                .insert(self.property_count, &Self::env().caller());
            self.property_count
        }

        #[ink(message)]
        pub fn approve_property(&mut self, property_id: u32) -> Result<String> {
            if !self.is_governance(Self::env().caller()) {
                return Err(Error::NotTheGovernanceError);
            }
            self.property_approved.insert(property_id, &true);
            return Ok(String::from("Approved"));
        }

        #[ink(message)]
        pub fn buy_property(&mut self, property_id: u32, price: u32) -> Result<()> {
            let caller = Self::env().caller();
            if self.property_price.get(property_id).unwrap_or_default() > price {
                return Err(Error::PriceTooLess);
            }
            if !self.property_approved.get(property_id).unwrap_or_default() {
                return Err(Error::PropertyNotApproved);
            }
            self.property_owner.insert(property_id, &caller);
            return Ok(());
        }
        #[ink(message)]
        pub fn is_governance(&self, caller: AccountId) -> bool {
            if self.governance == caller {
                return true;
            } else {
                return false;
            }
        }

        #[ink(message)]
        pub fn owner_of(&self, property_id: u32) -> AccountId {
            self.property_owner.get(property_id).unwrap_or_default()
        }
    }
}

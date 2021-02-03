#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Approve {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        value: Balance
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficentBalance,
        InsufficentAllowance,

    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances= StorageHashMap::new();
            balances.insert(caller, total_supply);

            let instance = Self {
                total_supply: total_supply,
                balances: balances,
                allowances: StorageHashMap::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowances.get(&(owner, spender)).unwrap_or(&0)
        }


        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = Self::env().caller();

            self.transfer_help(who, to, value)
        }

        fn transfer_help(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let _from_balance = self.balance_of(from);
            if _from_balance < value {
                return Err(Error::InsufficentBalance);
            }
            self.balances.insert(from, _from_balance - value);
            let _to_balance = self.balance_of(to);
            self.balances.insert(to, _to_balance + value);

            Self::env().emit_event(Transfer{
                from: from,
                to: to,
                value: value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let who = Self::env().caller();

            let _allowance_balance = self.allowance(from, who);
            if _allowance_balance < value {
                return Err(Error::InsufficentAllowance)
            }

            self.transfer_help(from, to, value)?;

            self.allowances.insert((from, who), _allowance_balance - value);

            Ok(())
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let who = Self::env().caller();

            self.allowances.insert((who, spender), value);
            Self::env().emit_event(Approve {
                from: who,
                to: spender,
                value: value,
            });

            Ok(())
        }


    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env;

        use ink_lang as ink;

        #[ink::test]
        fn create_contract_works() {
            let erc20 = Erc20::new(1000);
            assert_eq!(erc20.total_supply(), 1000);
        }

        #[ink::test]
        fn balance_of_works() {
            let erc20 = Erc20::new(1000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            assert_eq!(erc20.balance_of(accounts.alice), 1000);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = Erc20::new(1000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            assert_eq!(erc20.transfer(accounts.bob, 100), Ok(()));
            assert_eq!(erc20.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn tranfer_failed_insufficent() {
            let mut erc20 = Erc20::new(1000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            assert_eq!(erc20.transfer(accounts.bob, 100), Ok(()));
            assert_eq!(erc20.transfer(accounts.bob, 1000), Err(Error::InsufficentBalance));
        }

        #[ink::test]
        fn tranfer_approve_work() {
            let mut erc20 = Erc20::new(1000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");

            assert_eq!(erc20.approve(accounts.bob, 500), Ok(()));
            assert_eq!(erc20.allowance(accounts.alice, accounts.bob), 500);
            assert_eq!(erc20.allowance(accounts.bob, accounts.alice), 0);
        }

    }
}

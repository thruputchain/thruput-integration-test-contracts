//! An implementation of ERC20 without the storage layer.

// If the interface's `Amount` type changes away from `i128`, our casts
// below will be necessary
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::useless_conversion)]

use crate::interface::*;
use sdk::*;

#[cfg(feature = "checked")]
// use checked nnint operation to fail before commit
use sdk::non_negative_int_checked_inc as non_negative_int_inc;

// Compute keys for storing things

fn get_owner_key() -> StorageKey {
    b"owner".hash().into()
}

fn get_total_supply_key() -> StorageKey {
    b"total_supply".hash().into()
}

fn get_balance_key(addr: &ContractAddress) -> StorageKey {
    (b"balance", addr).hash().into()
}

fn get_allowance_key(
    owner: &ContractAddress,
    spender: &ContractAddress,
) -> StorageKey {
    debug_assert!(owner != spender, "no allowance needed on spending self");

    (b"allowance", owner, spender).hash().into()
}

// Read/write permanent state

fn get_owner() -> ContractAddress {
    bytestring_get_sized(&get_owner_key()).into()
}

fn ensure_caller_is_owner() {
    let caller = get_caller();
    let owner = get_owner();

    ensure(caller == owner, ERC20Error::MustBeContractOwner)
}

fn add_to_total_supply(diff: Difference) {
    non_negative_int_inc(
        &get_total_supply_key(),
        diff,
    )
}

fn add_to_balance(addr: &ContractAddress, diff: Difference) {
    non_negative_int_inc(
        &get_balance_key(addr),
        diff,
    )
}

fn add_to_allowance(
    owner: &ContractAddress,
    spender: &ContractAddress,
    diff: Difference,
) {
    non_negative_int_inc(
        &get_allowance_key(owner, spender),
        diff,
    )
}

#[derive(Default)]
pub struct Token;

#[contract(storage = false, version = "0.1.0")]
impl ERC20 for Token {
    fn initialize(&mut self, contract_owner: &ContractAddress) {
        let key = get_owner_key();

        // Make sure we haven't already been initialized
        ensure(!bytestring_exists(&key), ERC20Error::AlreadyInitialized);

        // Store ourselves
        bytestring_set(&key, contract_owner.as_ref());
    }

    fn transfer(
        &mut self,
        to: &ContractAddress,
        amount: Amount,
    ) {
        let caller = get_caller();
        let diff = amount_to_diff(amount);
        add_to_balance(&caller, -diff);
        add_to_balance(to, diff);

        log_transfer_event(&caller, to, amount);
    }

    fn transfer_from(
        &mut self,
        owner: &ContractAddress,
        receiver: &ContractAddress,
        amount: Amount,
    ) {
        let spender = get_caller();
        let diff = amount_to_diff(amount);
        add_to_allowance(owner, &spender, -diff);
        add_to_balance(owner, -diff);
        add_to_balance(receiver, diff);

        log_transfer_event(owner, receiver, amount);
    }

    fn adjust_allowance(
        &mut self,
        spender: &ContractAddress,
        diff: Difference,
    ) {
        let owner = get_caller();

        add_to_allowance(&owner, spender, diff);

        log_approval_event(&owner, spender, diff);
    }

    fn allowance(
        &self,
        owner: &ContractAddress,
        spender: &ContractAddress,
    ) -> Amount {
        non_negative_int_try_get(&get_allowance_key(owner, spender))
            .unwrap_or(0) as Amount
    }

    fn get_balance_of(&self, addr: &ContractAddress) -> Amount {
        non_negative_int_try_get(&get_balance_key(addr)).unwrap_or(0)
            as Amount
    }

    fn get_balance(&self) -> Amount {
        self.get_balance_of(&get_caller())
    }

    fn mint(&mut self, receiver: &ContractAddress, amount: Amount) {
        ensure_caller_is_owner();

        let diff = amount_to_diff(amount);
        add_to_balance(receiver, diff);
        add_to_total_supply(diff);

        log_transfer_event(CONTRACT_ADDR_ZERO, receiver, amount);
    }

    fn burn(&mut self, owner: &ContractAddress, amount: Amount) {
        ensure_caller_is_owner();

        let diff = amount_to_diff(amount);
        add_to_balance(owner, -diff);
        add_to_total_supply(-diff);

        log_transfer_event(owner, CONTRACT_ADDR_ZERO, amount);
    }

    fn total_supply(&self) -> Amount {
        non_negative_int_try_get(&get_total_supply_key()).unwrap_or(0)
            as Amount
    }
}

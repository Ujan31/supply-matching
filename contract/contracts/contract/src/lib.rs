#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Address};

// Struct for Supply
#[contracttype]
#[derive(Clone)]
pub struct Supply {
    pub supplier: Address,
    pub item: Symbol,
    pub quantity: u32,
}

// Struct for Demand
#[contracttype]
#[derive(Clone)]
pub struct Demand {
    pub demander: Address,
    pub item: Symbol,
    pub quantity: u32,
}

#[contract]
pub struct SupplyMatchingContract;

#[contractimpl]
impl SupplyMatchingContract {

    // Add supply
    pub fn add_supply(env: Env, supplier: Address, item: Symbol, quantity: u32) {
        let mut supplies: Vec<Supply> = env.storage().instance()
            .get(&Symbol::new(&env, "SUPPLIES"))
            .unwrap_or(Vec::new(&env));

        let supply = Supply {
            supplier,
            item,
            quantity,
        };

        supplies.push_back(supply);
        env.storage().instance().set(&Symbol::new(&env, "SUPPLIES"), &supplies);
    }

    // Add demand
    pub fn add_demand(env: Env, demander: Address, item: Symbol, quantity: u32) {
        let mut demands: Vec<Demand> = env.storage().instance()
            .get(&Symbol::new(&env, "DEMANDS"))
            .unwrap_or(Vec::new(&env));

        let demand = Demand {
            demander,
            item,
            quantity,
        };

        demands.push_back(demand);
        env.storage().instance().set(&Symbol::new(&env, "DEMANDS"), &demands);
    }

    // Match supply and demand (basic logic)
    pub fn match_supply(env: Env, item: Symbol) -> Vec<(Address, Address, u32)> {
        let supplies: Vec<Supply> = env.storage().instance()
            .get(&Symbol::new(&env, "SUPPLIES"))
            .unwrap_or(Vec::new(&env));

        let demands: Vec<Demand> = env.storage().instance()
            .get(&Symbol::new(&env, "DEMANDS"))
            .unwrap_or(Vec::new(&env));

        let mut matches: Vec<(Address, Address, u32)> = Vec::new(&env);

        for supply in supplies.iter() {
            if supply.item == item {
                for demand in demands.iter() {
                    if demand.item == item {
                        let matched_qty = if supply.quantity < demand.quantity {
                            supply.quantity
                        } else {
                            demand.quantity
                        };

                        matches.push_back((
                            supply.supplier.clone(),
                            demand.demander.clone(),
                            matched_qty,
                        ));
                    }
                }
            }
        }

        matches
    }
}

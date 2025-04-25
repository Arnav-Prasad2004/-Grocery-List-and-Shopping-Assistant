#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Vec, String};

// A struct to represent each grocery item
#[contracttype]
#[derive(Clone)]
pub struct GroceryItem {
    pub name: String,
    pub quantity: u32,
    pub is_bought: bool,
}

// Symbol for storage key
const GROCERY_LIST: Symbol = symbol_short!("GLIST");

#[contract]
pub struct GroceryListContract;

#[contractimpl]
impl GroceryListContract {
    // Add a new grocery item
    pub fn add_item(env: Env, name: String, quantity: u32) {
        let mut list: Vec<GroceryItem> = env
            .storage()
            .instance()
            .get(&GROCERY_LIST)
            .unwrap_or(Vec::new(&env));

        list.push_back(GroceryItem {
            name,
            quantity,
            is_bought: false,
        });

        env.storage().instance().set(&GROCERY_LIST, &list);
    }

    // Mark an item as bought by name
    pub fn mark_bought(env: Env, name: String) {
        let mut list: Vec<GroceryItem> = env
            .storage()
            .instance()
            .get(&GROCERY_LIST)
            .unwrap_or(Vec::new(&env));

        for i in 0..list.len() {
            let mut item = list.get_unchecked(i);
            if item.name == name {
                item.is_bought = true;
                list.set(i, item.clone());  // Use set() instead of set_unchecked
                break;
            }
        }

        env.storage().instance().set(&GROCERY_LIST, &list);
    }

    // Get the entire grocery list
    pub fn get_list(env: Env) -> Vec<GroceryItem> {
        env.storage()
            .instance()
            .get(&GROCERY_LIST)
            .unwrap_or(Vec::new(&env))
    }

    // Remove all bought items
    pub fn clear_bought(env: Env) {
        let list: Vec<GroceryItem> = env
            .storage()
            .instance()
            .get(&GROCERY_LIST)
            .unwrap_or(Vec::new(&env));

        let mut filtered = Vec::new(&env);  // Use Vec::new to create a new vector

        for item in list.iter() {
            if !item.is_bought {
                filtered.push_back(item.clone());
            }
        }

        env.storage().instance().set(&GROCERY_LIST, &filtered);
    }
}

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct WasteDeposit {
    pub id: u64,
    pub user: Address,
    pub weight_grams: u32,
    pub reward_amount: i128,
}

const DEPOSITS_KEY: Symbol = symbol_short!("DEPOSITS");

#[contract]
pub struct TrashToCashContract;

#[contractimpl]
impl TrashToCashContract {
    pub fn get_deposits(env: Env) -> Vec<WasteDeposit> {
        env.storage().instance().get(&DEPOSITS_KEY).unwrap_or(Vec::new(&env))
    }

    pub fn record_waste_deposit(env: Env, admin: Address, user: Address, weight_grams: u32, reward: i128) -> u64 {
        admin.require_auth();

        let mut deposits: Vec<WasteDeposit> = env.storage().instance().get(&DEPOSITS_KEY).unwrap_or(Vec::new(&env));
        let id = env.prng().gen::<u64>();
        let dep = WasteDeposit {
            id,
            user,
            weight_grams,
            reward_amount: reward,
        };

        deposits.push_back(dep);
        env.storage().instance().set(&DEPOSITS_KEY, &deposits);

        id
    }
}

mod test;

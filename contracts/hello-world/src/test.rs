#![cfg(test)]

use crate::{contract::ContractClient, storage::types::Asset, Contract};

use crate::token::token::{Token, TokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, String};

fn create_usdc_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register_contract(None, Token {}));
    token.initialize(admin, &7, &"USDC".into_val(e), &"USDC".into_val(e));
    token
}

#[test]
fn create_token_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let contract_client = ContractClient::new(&env, &contract_id);
    let asset_provider_address = Address::generate(&env);
    let client_address = Address::generate(&env);
    let admin = Address::generate(&env);
    let usdc_token = create_usdc_token(&env, &admin);

    let asset: Asset = Asset {
        title: String::from_str(&env, "Test Asset"),
        monthly_fee: 100,
        total: 1000,
        purchased: false,
        deadline: 0,
        next_due_date: 0,
        grace_period_end: 0,
        client: client_address,
        asset_provider: asset_provider_address,
        token: usdc_token.address,
    };

    contract_client.create_asset(&asset);

    let asset_result = contract_client.get_asset();

    assert_eq!(asset_result.client, asset.client);
    assert_eq!(asset_result.asset_provider, asset.asset_provider);
    assert_eq!(asset_result.token, asset.token);
    assert_eq!(asset_result.title, asset.title);
    assert_eq!(asset_result.monthly_fee, asset.monthly_fee);
    assert_eq!(asset_result.total, asset.total);
    assert_eq!(asset_result.purchased, asset.purchased);
    assert_eq!(asset_result.deadline, asset.deadline);
    assert_eq!(asset_result.next_due_date, asset.next_due_date);
    assert_eq!(asset_result.grace_period_end, asset.grace_period_end);
}

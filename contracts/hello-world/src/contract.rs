use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, BytesN, Env, Symbol, Val, Vec};

use crate::{error::ContractError, storage::types::{Asset, DataKey}};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn deploy(
        env: Env,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<Val>,
    ) -> (Address, Val) {
        if deployer != env.current_contract_address() {
            deployer.require_auth();
        }

        let deployed_address = env
            .deployer()
            .with_address(deployer, salt)
            .deploy(wasm_hash);

        let res: Val = env.invoke_contract(&deployed_address, &init_fn, init_args);
        (deployed_address, res)
    }

    pub fn create_asset(e: Env, asset_properties: Asset) -> Result<Asset, ContractError> {
        if e.storage().instance().has(&DataKey::Asset) {
            return Err(ContractError::AssetAlreadyCreated);
        }

        if asset_properties.total == 0 {
            return Err(ContractError::TotalCannotBeZero);
        }

        Ok(asset_properties)
    }

    pub fn pay_monthly_fee(e: Env, signer: Address, new_grace_period: u64) -> Result<(), ContractError> {
        let asset_result = Self::get_asset(e.clone());
        let mut asset = match asset_result {
            Ok(asset) => asset,
            Err(err) => return Err(err),
        };

        if signer != asset.asset_provider.address {
            return Err(ContractError::SignerIsNotAssetProvider);
        }
        signer.require_auth();

        if asset.grace_period_end > e.ledger().timestamp() {
            return Err(ContractError::GracePeriodHasBeenReached);
        }

        let usdc_token = TokenClient::new(&e, &asset.token);
        let signer_balance = usdc_token.balance(&signer);
        if signer_balance < asset.monthly_fee {
            return Err(ContractError::SignerHaveInsufficientBalance);
        }

        usdc_token.transfer(&asset.asset_provider.address, &asset.client.address, &asset.monthly_fee);

        asset.grace_period_end = new_grace_period;
        e.storage().instance().set(&DataKey::Asset, &asset);

        Ok(())
    }

    pub fn buy_asset(e: Env, signer: Address) -> Result<(), ContractError> {
        let asset_result = Self::get_asset(e.clone());
        let asset = match asset_result {
            Ok(asset) => asset,
            Err(err) => return Err(err),
        };

        if !asset.purchased {
            return Err(ContractError::AssetWithoutPurchaseOption);
        }

        if signer != asset.asset_provider.address {
            return Err(ContractError::SignerIsNotAssetProvider);
        }

        signer.require_auth();

        if asset.grace_period_end < e.ledger().timestamp() {
            return Err(ContractError::TimeLimitHasExpired);
        }

        let usdc_token = TokenClient::new(&e, &asset.token);
        let signer_balance = usdc_token.balance(&signer);
        if signer_balance < asset.total {
            return Err(ContractError::SignerHaveInsufficientBalance);
        }

        usdc_token.transfer(&asset.asset_provider.address, &asset.client.address, &asset.total);

        Ok(())
    }

    pub fn get_asset_by_contract_id(
        e: Env,
        contract_id: Address,
    ) -> Result<Asset, ContractError> {
        let args: Vec<Val> = Vec::new(&e);

        let result = e.invoke_contract::<Asset>(
            &contract_id,
            &Symbol::new(&e, "get_asset"),
            args.try_into().unwrap(),
        );

        Ok(result)
    }

    pub fn get_asset(e: Env) -> Result<Asset, ContractError> {
        let asset = e
            .storage()
            .instance()
            .get::<_, Asset>(&DataKey::Asset)
            .ok_or(ContractError::AssetNotFound);
        Ok(asset?)
    }
} 
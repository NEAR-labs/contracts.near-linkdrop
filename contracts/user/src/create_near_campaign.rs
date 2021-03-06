use crate::gas::*;
use crate::*;
use near_sdk::PublicKey;
use std::convert::TryFrom;

#[near_bindgen]
impl User {
  #[payable]
  #[private]
  pub fn create_near_campaign(
    &mut self,
    name: String,
    public_key: PublicKey,
    total_keys: u64,
    tokens_per_key: U128,
    account_creator: AccountId,
  ) -> Promise {
    assert!(!name.contains('.'));

    let campaign_id = AccountId::try_from(format!("{}.{}", name, env::current_account_id()))
      .expect("Name is expected to be a valid subaccount prefix");

    Promise::new(campaign_id.clone())
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(NEAR_CAMPAIGN_WASM.to_vec())
      .function_call(
        "new".to_string(),
        json!({
          "campaign_id": self.future_campaign_id,
          "total_keys": total_keys,
          "tokens_per_key": tokens_per_key,
          "account_creator": account_creator,
          "user_id": env::current_account_id()
        })
        .to_string()
        .into_bytes(),
        0,
        t_gas(50), // 50 TGas
      )
      .then(ext_self_user::on_near_campaign_created(
        campaign_id,
        env::current_account_id(),
        0,
        BASE_GAS, // 25 TGas
      ))
  }
}

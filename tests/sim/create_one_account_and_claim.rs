use crate::utils::{init_external_linkdrop, init_near_campaign, KeySet};
use near_crypto::{InMemorySigner, Signer};
use near_sdk::serde_json::json;
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_one_account_and_claim() {
  let (root, mut near_campaign) = init_near_campaign(1, "5");
  init_external_linkdrop(&root);

  let key_set = KeySet::create(0, 0);
  let (_, _, sk) = key_set.some_keys(0);
  let (new_pk, johns_pk, _) = KeySet::create(1, 1).some_keys(0);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys())
  );

  // We want to sing transaction by new key;
  let claim_signer = InMemorySigner::from_secret_key(near_campaign.account_id().into(), sk);
  near_campaign.user_account.signer = claim_signer.clone();

  // Create a new account
  let result = near_campaign.user_account.call(
    near_campaign.account_id().clone(),
    "create_account_and_claim",
    &json!({
      "new_account_id": "john.testnet".to_string(),
      "new_public_key": new_pk
    })
      .to_string()
      .into_bytes(),
    100000000000000, // 100 TGas
    0
  );

  {
    let runtime = root.borrow_runtime();

    // The new account should exist with 5 NEAR on the balance
    let john = runtime.view_account("john.testnet");
    assert_eq!(to_yocto("5"), john.unwrap().amount);

    // Verify that the key has been added to the new account
    let johns_key = runtime.view_access_key("john.testnet", &johns_pk);
    assert_eq!(johns_key.is_some(), true);

    // Used key should not exist after the successful 'claim'
    let key = runtime.view_access_key(
      near_campaign.account_id().as_str(),
      &claim_signer.public_key(),
    );
    assert_eq!(key.is_none(), true);

    // Check the log for callback output
    assert_eq!(result.logs().len(), 1);
    assert!(result.logs()[0].contains("The account is created and link is claimed: true"));
  }
}

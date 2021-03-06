use crate::utils::{init_near_campaign, KeySet};
use near_crypto::{InMemorySigner, Signer};
use near_sdk::serde_json::json;
use near_sdk_sim::{call, to_yocto};

#[test]
fn claim_one_link() {
  let (root, mut near_campaign) = init_near_campaign(1, "5");
  let bob = root.create_user("bob".parse().unwrap(), to_yocto("10"));
  let key_set = KeySet::create(0, 0);
  let (_, pk, sk) = key_set.some_keys(0);

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(key_set.public_keys())
  );

  // Check if the key was added as functional call assess key
  {
    let runtime = root.borrow_runtime();
    let key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk);
    assert_eq!(key.is_some(), true);
  }

  // We want to sing transaction by new key;
  let claim_signer = InMemorySigner::from_secret_key(near_campaign.account_id().into(), sk);

  near_campaign.user_account.signer = claim_signer.clone();

  let result = near_campaign.user_account.call(
    near_campaign.account_id().clone(),
    "claim",
    &json!({
      "account_id": bob.account_id().to_string()
    })
      .to_string()
      .into_bytes(),
    100000000000000, // 100 TGas
    0
  );
  result.assert_success();

  // Used key should not exist after the successful 'claim'
  {
    let runtime = root.borrow_runtime();
    let key = runtime.view_access_key(
      near_campaign.account_id().as_str(),
      &claim_signer.public_key(),
    );
    assert_eq!(key.is_none(), true);

    // Check Bob balance
    assert_eq!(to_yocto("15"), bob.account().unwrap().amount);

    // Check the log for callback output
    assert_eq!(result.logs().len(), 1);
    assert!(result.logs()[0].contains("The link is claimed: true"));
  }
}

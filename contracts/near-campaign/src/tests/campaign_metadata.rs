use super::utils::{create_campaign, get_context, keys};
use crate::*;
use near_sdk::testing_env;

#[test]
fn campaign_metadata() {
  let keys = keys::get_public_keys(0, 9);
  testing_env!(get_context());
  let last_block_timestamp = env::block_timestamp();

  let mut contract = create_campaign();
  // It is planned to add 10 keys
  contract.keys_stats.total = 10;
  contract.add_keys(keys);
  // The 'create_account_and_claim' method was executed for one key
  contract.keys_stats.active -= 1;
  contract.keys_stats.created += 1;

  let metadata = contract.get_campaign_metadata();

  assert_eq!(
    U128::from(1_000_000_000_000_000_000_000_000),
    metadata.tokens_per_key
  );
  assert!(metadata.created_at >= last_block_timestamp);
  assert_eq!(CampaignStatus::Active, metadata.status);
  assert_eq!(1, metadata.campaign_id);
  assert_eq!(
    AccountId::new_unchecked("testnet".to_string()),
    metadata.account_creator
  );
  assert_eq!(
    AccountId::new_unchecked("alice.linkdrop.testnet".to_string()),
    metadata.user_id
  );
  assert_eq!("1.0".to_string(), metadata.version);
  assert_eq!(9, metadata.keys_stats.active);
  assert_eq!(1, metadata.keys_stats.created);
}

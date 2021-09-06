#![allow(
  unused_must_use,
  unused_imports,
  unused_mut,
  dead_code,
  unused_variables
)]
use crate::utils::{get_public_keys, init};
use near_crypto::{InMemorySigner, PublicKey, SecretKey, Signer};
use near_sdk_sim::{call, to_yocto, view};
use std::str::FromStr;

// TODO Need to finish

// Test the that the add_keys will works correctly with the predefined keys amount

#[test]
fn add_keys_fail() {
  let (root, mut near_campaign) = init("5");
  let public_keys = get_public_keys(0, 49);

  let res = call!(
    near_campaign.user_account,
    near_campaign.add_keys(public_keys)
  );

  // 300000000000000

  // 108594692914512
  // 001665668272053

  // 055131666112497
  // 055132786365708

  // dbg!(res.gas_burnt() / 1_000_000_000_000);
  // dbg!(res.promise_errors());
  // dbg!(res.logs());

  // let res1 = view!(near_campaign.get_keys(get_public_keys(89, 99)));
  let res1 = view!(near_campaign.get_campaign_metadata());
  dbg!(res1.unwrap_json_value());

  // {
  //   let mut a = root.borrow_runtime_mut();
  //   dbg!(a.current_block());
  //   // dbg!(a)
  // }
}

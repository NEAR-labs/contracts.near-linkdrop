use crate::utils::{CommonUtils, UserCreatorUtility};
use near_sdk_sim::{call, to_yocto};

#[test]
fn create_user_name_is_already_exists() {
  let alice_initial_balance = "200";
  let transfer_amount = to_yocto("100");

  let (root, runtime) = CommonUtils::init_simulation();
  let user_creator = UserCreatorUtility::init(root, alice_initial_balance);

  // Pre-create Alice Linkdrop account
  user_creator.create_drop_user();

  let contract = user_creator.contract;
  let alice = user_creator.user;
  let pk = user_creator.public_key;
  let contract_balance_start = contract.account().unwrap().amount;

  let result = call!(
    alice,
    contract.create_user_account(alice.account_id.to_string(), pk.as_pk1()),
    deposit = transfer_amount
  );
  result.assert_success();

  // One error must occur while running the method
  CommonUtils::assert_one_promise_error(
    result.clone(),
    "Can't create a new account \"alice.linkdrop\", because it already exists"
  );

  // Check the log for callback output
  assert_eq!(result.logs().len(), 1);
  assert!(result.logs()[0].contains("Is user created: false"));

  // Alice's balance has not changed
  let alice_balance = CommonUtils::retrieve_account_balance(alice.account_id.as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(to_yocto(alice_initial_balance), alice_balance);

  // The balance of the contract has not changed
  let contract_balance_end = CommonUtils::retrieve_account_balance(contract.account_id().as_str(), &runtime);
  CommonUtils::assert_eq_with_gas(contract_balance_start, contract_balance_end);
}
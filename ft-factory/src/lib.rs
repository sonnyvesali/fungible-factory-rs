use near_sdk::{env, near_bindgen, AccountId, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
const CODE: &'static [u8] = include_bytes!("FT.wasm");

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct TokenFactory {
    main_account_id: AccountId,
}


#[near_bindgen]
impl TokenFactory {
    
#[result_serializer(borsh)]
   pub fn new() -> Self {
       Self {
            main_account_id: env::signer_account_id(),
       }
    }


    #[payable]
    pub fn deploy_child_token_contract(&mut self, prefix: AccountId) -> Promise {
        let subaccount_id =format!("{}.{}", prefix, self.main_account_id);
        Promise::new(subaccount_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(env::attached_deposit())
            .deploy_contract(CODE.to_vec())
    }
}

// ok so now it compiles now it's time to write some tests to make sure it performs as expected, unit tests should be able to cover all 
// of the bases for this simple contract once we get it passing


#[cfg(all(test, not(target_arch="wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;

    // What does the context need to include for these tests ? 
    //    


}

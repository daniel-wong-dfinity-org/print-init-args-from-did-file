// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct Init { pub i: Option<u64> }

pub struct Service(pub Principal);
impl Service {
  pub async fn do_stuff(&self) -> Result<()> {
    ic_cdk::call(self.0, "do_stuff", ()).await
  }
}


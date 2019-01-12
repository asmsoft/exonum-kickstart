pub mod storage;

use exonum::{
    api::ServiceApiBuilder,
};

#[derive(Clone)]
pub struct PublicApi;

impl PublicApi {
    pub fn wire(_builder: &mut ServiceApiBuilder) {}
}

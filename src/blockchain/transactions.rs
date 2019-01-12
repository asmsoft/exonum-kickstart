// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

use exonum::{
    blockchain::{ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use blockchain::{
    schema::Schema,
    errors::Error,
};

use crate::proto;

use SERVICE_ID;

/// Stores value by key
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxStore", serde_pb_convert)]
pub struct TxStore {
    pub key: String,
    pub val: String,
}

impl Transaction for TxStore {
    fn execute(&self, mut ctx: TransactionContext) -> ExecutionResult {
        let mut schema = Schema::new(ctx.fork());
        if schema.has_key(&self.key) {
            Err(Error::AlreadyExists)?;
        }

        schema.store(&self.key, self.val.to_owned());
        Ok(())
    }
}

impl TxStore {
    #[doc(hidden)]
    pub fn sign(key: &str, val: &str, pk: &PublicKey, sk: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(
            Self {
                key: key.to_owned(),
                val: val.to_owned(),
            },
            SERVICE_ID,
            *pk,
            sk,
        )
    }
}

/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum {{project_name}}Transactions {
    /// Store tx.
    Store(TxStore)
}


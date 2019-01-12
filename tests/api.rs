extern crate exonum;
extern crate {{project_name}};
extern crate exonum_testkit;
extern crate exonum_configuration;
#[macro_use]
extern crate serde_json;

use exonum_testkit::{
    TestKitApi,
    TestKit,
    TestKitBuilder,
};

use exonum::{
    crypto::{
        self,
        Hash,
    },
    api::node::public::explorer::TransactionQuery,
    messages::{RawTransaction, Signed},
};
use exonum_testkit::ApiKind;
use {{project_name}}::{
    SERVICE_NAME,
    {{project_name}}Service,
    blockchain::transactions::{TxStore},
    api::storage::LoadRequest
};

#[test]
fn store_test() {
    let (mut testkit, api) = create_testkit();
    let tx = api.store("test_key", "test_val");
    testkit.create_block();
    api.assert_tx_success(tx.hash());
    assert_eq!("test_val", api.load("test_key"));
}

struct {{project_name}}Api {
    pub inner: TestKitApi,
}

impl {{project_name}}Api {
    fn store(&self, key: &str, val: &str) -> Signed<RawTransaction> {
        let (pubkey, sec) = crypto::gen_keypair();
        let tx = TxStore::sign(key, val, &pubkey, &sec);
        let tx_info: serde_json::Value = self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&tx)
            .post("v1/store")
            .unwrap();
        assert_eq!(tx_info, json!({ "tx_hash": tx.hash() }));
        tx
    }

    fn load(&self, key: &str) -> String {
        self.inner
            .public(ApiKind::Service(SERVICE_NAME))
            .query(&LoadRequest { key: key.to_owned() })
            .get("v1/load")
            .unwrap()
    }

    fn assert_tx_status(&self, tx_hash: Hash, expected_status: &serde_json::Value) {
        let info: serde_json::Value = self.inner
            .public(ApiKind::Explorer)
            .query(&TransactionQuery::new(tx_hash))
            .get("v1/transactions")
            .unwrap();

        if let serde_json::Value::Object(mut info) = info {
            let tx_status = info.remove("status").unwrap();
            assert_eq!(tx_status, *expected_status);
        } else {
            panic!("Invalid transaction info format, object expected");
        }
    }

    fn assert_tx_success(&self, tx_hash: Hash) {
        self.assert_tx_status(tx_hash, &json!({ "type": "success" }));
    }
}

fn create_testkit() -> (TestKit, {{project_name}}Api) {
    let testkit = TestKitBuilder::validator()
        .with_service({{project_name}}Service)
        .create();

    let api = {{project_name}}Api {
        inner: testkit.api(),
    };
    (testkit, api)
}

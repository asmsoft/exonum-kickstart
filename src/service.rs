use api::PublicApi;
use exonum::{
    api::ServiceApiBuilder,
    helpers::fabric::{ServiceFactory, Context},
    crypto::Hash,
    storage::Snapshot,
    blockchain::{TransactionSet, Transaction, Service},
    messages::RawTransaction,
};
use blockchain::{
    transactions::{{project_name}}Transactions,
    schema::Schema,
};

pub const SERVICE_ID: u16 = 128;
pub const SERVICE_NAME: &str = "{{project_name}}";

#[derive(Debug, Default)]
pub struct {{project_name}}Service;

impl {{project_name}}Service {
    pub fn new() -> {{project_name}}Service {
        {{project_name}}Service
    }
}

impl Service for {{project_name}}Service {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &'static str {
        SERVICE_NAME
    }

    fn state_hash(&self, view: &Snapshot) -> Vec<Hash> {
        let schema = Schema::new(view);
        schema.state_hash()
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
        {{project_name}}Transactions::tx_from_raw(raw).map(Into::into)
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}

impl ServiceFactory for {{project_name}}Service {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new({{project_name}}Service::new())
    }
}

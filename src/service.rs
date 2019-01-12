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
    transactions::project_nameTransactions,
    schema::Schema,
};

pub const SERVICE_ID: u16 = 128;
pub const SERVICE_NAME: &str = "{{project_name}}";

#[derive(Debug, Default)]
pub struct project_nameService;

impl project_nameService {
    pub fn new() -> project_nameService {
        project_nameService
    }
}

impl Service for project_nameService {
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
        project_nameTransactions::tx_from_raw(raw).map(Into::into)
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        PublicApi::wire(builder);
    }
}

impl ServiceFactory for project_nameService {
    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(project_nameService::new())
    }
}

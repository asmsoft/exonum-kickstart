extern crate exonum;
extern crate exonum_configuration;
extern crate project_name;

use exonum::helpers::fabric::NodeBuilder;

use exonum_configuration::ServiceFactory;
use project_name::project_nameService;

fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service(Box::new(ServiceFactory))
        .with_service(Box::new(project_nameService::new()))
        .run();
}
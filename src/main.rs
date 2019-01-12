extern crate exonum;
extern crate exonum_configuration;
{%- if use_exonum_time %}
extern crate exonum_time;
{%- endif %}
extern crate {{project_name}};

use exonum::helpers::fabric::NodeBuilder;

use exonum_configuration::ServiceFactory;
use {{project_name}}::{{project_name}}Service;

fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        {%- if use_exonum_time %}
        .with_service(Box::new(exonum_time::TimeServiceFactory))
        {%- endif %}
        .with_service(Box::new(ServiceFactory))
        .with_service(Box::new({{project_name}}Service::new()))
        .run();
}
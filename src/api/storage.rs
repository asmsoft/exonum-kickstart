use exonum::api::{
    Error as ApiError,
    Result as ApiResult,
    ServiceApiState,
    ServiceApiBuilder
};

use blockchain::schema::Schema;

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct LoadRequest {
    pub key: String,
}

fn load(state: &ServiceApiState, query: LoadRequest) -> ApiResult<String> {
    let snapshot = state.snapshot();
    let schema = Schema::new(&snapshot);

    if let Some(val) = schema.load(&query.key) {
        Ok(val)
    } else {
        Err(ApiError::NotFound("Value not found.".to_string()))?
    }
}

pub fn wire_api(builder: &mut ServiceApiBuilder) {
    builder
        .public_scope()
        .endpoint("/v1/load", load);
}
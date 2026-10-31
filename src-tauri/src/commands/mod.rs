use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct EmptyRequest {}
#[derive(Serialize)]
pub struct Bootstrap {
    version: &'static str,
    ready: bool,
}
#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Success { ok: bool, value: T },
}
#[tauri::command]
pub fn app_bootstrap(request: EmptyRequest) -> ApiResult<Bootstrap> {
    let _ = request;
    ApiResult::Success {
        ok: true,
        value: Bootstrap {
            version: env!("CARGO_PKG_VERSION"),
            ready: true,
        },
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn bootstrap_envelope() {
        let result = super::app_bootstrap(super::EmptyRequest {});
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["ok"], true);
        assert_eq!(value["value"]["ready"], true);
    }
}

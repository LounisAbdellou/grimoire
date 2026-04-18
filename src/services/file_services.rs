use dioxus::prelude::*;

#[post("/api/files")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

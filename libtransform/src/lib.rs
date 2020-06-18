pub struct TransformResult {
    key: String,
    body: String,
}

#[no_mangle]
pub fn transform(message: String) -> Box<TransformResult> {
    Box::new(TransformResult {
        key: String::from("changing"),
        body: message,
    })
}

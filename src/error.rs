#[derive(Debug, Fail)]
pub enum ResourceError {
    #[fail(display = "method not allowed")] MethodNotAllowed,
}

#[macro_export]
macro_rules! status_custom {
    ($response:ident) => {
        status::Custom(
            Status::from_code($response.status_code).unwrap(),
            Json($response.response),
        )
    };
}
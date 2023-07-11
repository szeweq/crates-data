
macro_rules! impl_error {
    ($($enum:ident $tp:ty as $str:literal),*) => {
        #[derive(Debug)]
        pub enum GenError { $($enum($tp),)* }

        impl std::error::Error for GenError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    $(GenError::$enum(e) => Some(e),)*
                }
            }
        }
        impl std::fmt::Display for GenError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(GenError::$enum(e) => write!(f, "{} error: {}", $str, e),)*
                }
            }
        }
        $(impl From<$tp> for GenError {
            fn from(e: $tp) -> Self { GenError::$enum(e) }
        })*
    }
}
impl_error!(
    Io std::io::Error as "IO",
    Json serde_json::Error as "JSON"
);
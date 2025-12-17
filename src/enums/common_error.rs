use std::fmt::Formatter;

/// Common data error
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
#[cfg_attr(test, derive())]
#[allow(non_camel_case_types)]
pub enum CommonError {
    NO_VALID_INPUT_OR_PARAMETER,
    AWS_ACCESS_ERROR,
}

/// [std::fmt::Display] trait implementation
impl std::fmt::Display for CommonError {
    /// just to simplify format in log statements + generate to_string fn
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use std::fmt::Debug;

#[derive(Debug, Fail)]
pub enum MockServiceError<A>
where
    A: Debug,
{
    #[fail(display = "received an unexpected request: {:?}", _0)]
    UnexpectedRequest(A),
}

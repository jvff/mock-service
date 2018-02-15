extern crate async_server;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate tokio_service;

mod error;

mod finite_mock_service;
mod mock_action;
mod mock_service;
mod mock_service_handle;

mod repeated_reply_action;
mod single_reply_action;

mod hash_mock_service_handle;

pub use error::MockServiceError;

pub use finite_mock_service::FiniteMockService;
pub use mock_action::MockAction;
pub use mock_service::MockService;
pub use mock_service_handle::MockServiceHandle;

pub use repeated_reply_action::RepeatedReplyAction;
pub use single_reply_action::SingleReplyAction;

pub use hash_mock_service_handle::HashMockServiceHandle;

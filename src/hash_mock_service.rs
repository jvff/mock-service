use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::IntoFuture;
use futures::future::FutureResult;
use tokio_service::Service;

use super::error::MockServiceError;
use super::hash_mock_service_data::HashMockServiceData;
use super::mock_action::MockAction;
use super::mock_service::MockService;
use super::mock_service_handle::MockServiceHandle;

#[derive(Clone)]
pub struct HashMockService<A, B>
where
    A: Debug + Eq + Hash,
{
    data: Arc<Mutex<HashMockServiceData<A, B>>>,
}

impl<A, B> HashMockService<A, B>
where
    A: Debug + Eq + Hash,
{
    pub fn new() -> Self {
        HashMockService {
            data: Arc::new(Mutex::new(HashMockServiceData::new())),
        }
    }

    fn lock(&self) -> MutexGuard<HashMockServiceData<A, B>> {
        self.data.lock().expect(
            concat!(
                "an other thread panicked while holding a lock to the ",
                "HashMockService request map",
            ),
        )
    }
}

impl<A, B> Service for HashMockService<A, B>
where
    A: Debug + Eq + Hash,
{
    type Request = A;
    type Response = B;
    type Error = MockServiceError<A>;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.lock().call(request).into_future()
    }
}

impl<A, B> MockServiceHandle for HashMockService<A, B>
where
    A: Debug + Eq + Hash,
{
    type Request = A;
    type Response = B;

    fn set_action(
        &mut self,
        request: Self::Request,
        action:
            Box<MockAction<Request = Self::Request, Response = Self::Response>>,
    ) {
        self.lock().set_action(request, action)
    }

    fn remove_action(&mut self, request: Self::Request) {
        self.lock().remove_action(request);
    }
}

impl<A, B> MockService for HashMockService<A, B>
where
    A: Debug + Eq + Hash,
{
}

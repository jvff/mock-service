use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::error::MockServiceError;
use super::hash_mock_service_handle::HashMockServiceHandle;
use super::mock_action::MockAction;
use super::mock_service_handle::MockServiceHandle;

pub struct HashMockServiceData<A, B>
where
    A: Debug + Eq + Hash,
{
    request_map: HashMap<A, Box<MockAction<Request = A, Response = B>>>,
}

impl<A, B> HashMockServiceData<A, B>
where
    A: Debug + Eq + Hash,
{
    pub fn new() -> Self {
        HashMockServiceData {
            request_map: HashMap::new(),
        }
    }

    pub fn call(&mut self, request: A) -> Result<B, MockServiceError<A>> {
        let mut handle = HashMockServiceHandle::new();
        let result = self.try_call(request, &mut handle);

        handle.update_request_map(&mut self.request_map);

        result
    }

    fn try_call(
        &mut self,
        request: A,
        handle: &mut HashMockServiceHandle<A, B>,
    ) -> Result<B, MockServiceError<A>> {
        match self.request_map.get_mut(&request) {
            Some(action) => Ok(action.act(request, handle)),
            None => Err(MockServiceError::UnexpectedRequest(request)),
        }
    }
}

impl<A, B> MockServiceHandle for HashMockServiceData<A, B>
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
        self.request_map.insert(request, action);
    }

    fn remove_action(&mut self, request: Self::Request) {
        self.request_map.remove(&request);
    }
}

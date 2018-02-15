use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::error::MockServiceError;
use super::hash_mock_service_handle::HashMockServiceHandle;
use super::mock_action::MockAction;

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

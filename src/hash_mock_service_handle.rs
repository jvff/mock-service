use std::collections::HashMap;
use std::hash::Hash;

use super::mock_action::MockAction;
use super::mock_service_handle::MockServiceHandle;

enum Change<A, B> {
    Set(A, Box<MockAction<Request = A, Response = B>>),
    Remove(A),
}

pub struct HashMockServiceHandle<A, B>
where
    A: Eq + Hash,
{
    changes: Vec<Change<A, B>>,
    finished: bool,
}

impl<A, B> HashMockServiceHandle<A, B>
where
    A: Eq + Hash,
{
    pub fn new() -> Self {
        HashMockServiceHandle {
            changes: Vec::new(),
            finished: false,
        }
    }

    pub fn update_request_map(
        self,
        map: &mut HashMap<A, Box<MockAction<Request = A, Response = B>>>,
    ) {
        for change in self.changes {
            match change {
                Change::Set(request, response) => map.insert(request, response),
                Change::Remove(request) => map.remove(&request),
            };
        }
    }

    pub fn has_finished(&self) -> bool {
        self.finished
    }
}

impl<A, B> MockServiceHandle for HashMockServiceHandle<A, B>
where
    A: Eq + Hash,
{
    type Request = A;
    type Response = B;

    fn set_action(
        &mut self,
        request: Self::Request,
        action:
            Box<MockAction<Request = Self::Request, Response = Self::Response>>,
    ) {
        self.changes.push(Change::Set(request, action));
    }

    fn remove_action(&mut self, request: Self::Request) {
        self.changes.push(Change::Remove(request));
    }

    fn mark_finished(&mut self) {
        self.finished = true;
    }
}

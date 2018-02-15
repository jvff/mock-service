use super::mock_action::MockAction;

pub trait MockServiceHandle {
    type Request;
    type Response;

    fn set_action(
        &mut self,
        request: Self::Request,
        action:
            Box<MockAction<Request = Self::Request, Response = Self::Response>>,
    );

    fn remove_action(&mut self, request: Self::Request);
}

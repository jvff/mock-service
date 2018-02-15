use super::mock_service_handle::MockServiceHandle;

pub trait MockAction {
    type Request;
    type Response;

    fn act(
        &mut self,
        request: Self::Request,
        mock_service:
            &mut MockServiceHandle<
                Request = Self::Request,
                Response = Self::Response,
            >,
    ) -> Self::Response;
}

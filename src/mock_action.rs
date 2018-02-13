use super::mock_service::MockService;

pub trait MockAction<S>
where
    S: MockService,
{
    fn act(
        &mut self,
        request: &S::Request,
        mock_service: &mut S,
    ) -> S::Response;
}

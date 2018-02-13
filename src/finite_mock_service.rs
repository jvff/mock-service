use super::mock_service::MockService;

pub trait FiniteMockService: MockService {
    fn mark_finished(&mut self);
}

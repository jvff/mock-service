use async_server::FiniteService;

use super::mock_service::MockService;

pub trait FiniteMockService: FiniteService + MockService {
    fn mark_finished(&mut self);
}

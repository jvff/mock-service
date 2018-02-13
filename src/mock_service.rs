use tokio_service::Service;

use super::mock_action::MockAction;

pub trait MockService: Service {
    fn set_action(
        &mut self,
        request: <Self as Service>::Request,
        action: &mut MockAction<Self>,
    );

    fn remove_action(&mut self, request: <Self as Service>::Request);
}

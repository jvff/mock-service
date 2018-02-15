use tokio_service::Service;

use super::mock_service_handle::MockServiceHandle;

pub trait MockService:
    Service
        + MockServiceHandle<
            Request = <Self as Service>::Request,
            Response = <Self as Service>::Response,
        >
{
}

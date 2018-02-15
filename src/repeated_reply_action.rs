use std::marker::PhantomData;

use super::mock_action::MockAction;
use super::mock_service_handle::MockServiceHandle;

pub struct RepeatedReplyAction<A, B>
where
    B: Clone,
{
    _request: PhantomData<A>,
    reply: B,
}

impl<A, B> From<B> for RepeatedReplyAction<A, B>
where
    B: Clone,
{
    fn from(reply: B) -> Self {
        RepeatedReplyAction {
            _request: PhantomData,
            reply,
        }
    }
}

impl<A, B> MockAction for RepeatedReplyAction<A, B>
where
    B: Clone,
{
    type Request = A;
    type Response = B;

    fn act(
        &mut self,
        _request: A,
        _service:
            &mut MockServiceHandle<
                Request = Self::Request,
                Response = Self::Response,
            >,
    ) -> B {
        self.reply.clone()
    }
}

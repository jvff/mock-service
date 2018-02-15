use std::marker::PhantomData;

use super::mock_action::MockAction;
use super::mock_service_handle::MockServiceHandle;

pub struct SingleReplyAction<A, B> {
    _request: PhantomData<A>,
    reply: Option<B>,
}

impl<A, B> From<B> for SingleReplyAction<A, B> {
    fn from(reply: B) -> Self {
        SingleReplyAction {
            _request: PhantomData,
            reply: Some(reply),
        }
    }
}

impl<A, B> MockAction for SingleReplyAction<A, B> {
    type Request = A;
    type Response = B;

    fn act(
        &mut self,
        request: A,
        service:
            &mut MockServiceHandle<
                Request = Self::Request,
                Response = Self::Response,
            >,
    ) -> B {
        service.remove_action(request);

        self.reply.take().expect(concat!(
            "can't reply twice using SingleReplyAction, use ",
            "RepeatedReplyAction instead",
        ))
    }
}

use super::mock_action::MockAction;
use super::mock_service::MockService;

pub struct RepeatedReplyAction<T>
where
    T: Clone,
{
    reply: T,
}

impl<T> From<T> for RepeatedReplyAction<T>
where
    T: Clone,
{
    fn from(reply: T) -> Self {
        RepeatedReplyAction {
            reply,
        }
    }
}

impl<S> MockAction<S> for RepeatedReplyAction<S::Response>
where
    S: MockService,
    S::Response: Clone,
{
    fn act(&mut self, _request: &S::Request, _service: &mut S) -> S::Response {
        self.reply.clone()
    }
}

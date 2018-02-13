use super::mock_action::MockAction;
use super::mock_service::MockService;

pub struct SingleReplyAction<T> {
    reply: Option<T>,
}

impl<T> From<T> for SingleReplyAction<T> {
    fn from(reply: T) -> Self {
        SingleReplyAction {
            reply: Some(reply),
        }
    }
}

impl<S> MockAction<S> for SingleReplyAction<S::Response>
where
    S: MockService,
{
    fn act(&mut self, request: &S::Request, service: &mut S) -> S::Response {
        service.remove_action(request);

        self.reply.take().expect(concat!(
            "can't reply twice using SingleReplyAction, use ",
            "RepeatedReplyAction instead",
        ))
    }
}

pub trait MockAction {
    type Response;

    fn act(&mut self) -> Self::Response;
}

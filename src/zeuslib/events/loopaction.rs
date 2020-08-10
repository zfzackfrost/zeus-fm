#[derive(PartialEq)]
pub enum EventLoopAction {
    ContinueLoop,
    QuitLoop,
}
unsafe impl Sync for EventLoopAction {}

pub trait CommandTrait {
    fn command(&self) -> &str;
    fn name(&self) -> &str;
}

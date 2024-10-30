use crate::parser::Variable;

pub trait CommandTrait {
    fn command(&self) -> &str;
    fn name(&self) -> &str;
    fn is_applicable(&self, variables: &[Variable]) -> Result<bool, String>;
}

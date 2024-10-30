use crate::parser::Condition;

pub trait ConditionTrait {
    fn get_conditions(&self) -> Option<&[Condition]>;
}

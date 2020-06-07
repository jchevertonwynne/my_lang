use crate::lib::{Expression, Construct};

pub enum Line {
    Assignment(String, Expression),
    Expression(Expression),
    Construct(Construct)
}
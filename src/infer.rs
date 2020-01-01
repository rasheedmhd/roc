use crate::collections::ImMap;
use crate::solve;
use crate::subs::{Content, Subs, Variable};
use crate::types::{Constraint, Problem};

pub fn infer_expr(
    subs: &mut Subs,
    problems: &mut Vec<Problem>,
    constraint: &Constraint,
    expr_var: Variable,
) -> Content {
    solve::run(&ImMap::default(), problems, subs, constraint);

    subs.get(expr_var).content
}

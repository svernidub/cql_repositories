use std::ops::Deref;

use super::column_expr::Column;

use super::select_expr::*;
use super::value_expr::*;

#[derive(Clone)]
pub struct Where(pub Vec<WhereExpr>);

#[derive(Clone)]
pub enum WhereExpr {
    InExpression(Column, InExpr),
    WhereExpression(Column, Operator, Value)
}
use WhereExpr::*;

#[derive(Clone)]
pub struct InExpr(pub Vec<Value>);

#[derive(Clone)]
pub struct Operator(pub String);

impl Where {
    pub fn to_cql(&self) -> String {
        let Self(exprs) = self.clone();

        if exprs.is_empty() {
            return String::new()
        }

        let conds = exprs.into_iter().map(|expr| expr.to_cql()).collect::<Vec<String>>().join(" AND ");
        format!("WHERE {}", conds)
    }
}

impl WhereExpr {
    fn to_cql(&self) -> String {
        match self {
            InExpression(Column(c), inexpr) => {
                format!("{} {}", c, inexpr.to_clq())
            }
            WhereExpression(Column(c), op, v) => {
                format!("{} {} {}", c, op.to_clq(), v.to_cql())
            }
        }
    }
}

impl InExpr {
    fn to_clq(&self) -> String {
        let InExpr(xs) = self.clone();
        let arr = xs.into_iter().map(|x| x.to_cql()).collect::<Vec<String>>().join(", ");
        format!("IN ({})", arr)
    }
}

impl Operator {
    fn to_clq(&self) -> String {
        let Operator(string) = self.clone();
        string.clone()
    }
}

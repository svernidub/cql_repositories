use std::any::Any;
use std::collections::HashSet;

use super::column_expr::{AnyColumn, Column};
use super::columns_expr::Columns;
use super::order_expr::Order;

use super::from_expr::FromExpr;
use super::value_expr::Value;
use super::where_expr::*;

#[derive(Clone)]
pub struct SelectExpression {
    pub(crate) columns: Columns,
    pub(crate) from: FromExpr,

    pub(crate) where_: Option<Where>,
    pub(crate) order_by: Option<Order>,
    pub(crate) limit: Option<Limit>,
    pub(crate) allow_filtering: bool,
}

#[derive(Clone)]
pub struct Limit(pub u64);

impl SelectExpression {
    pub fn to_cql(&self) -> String {
        let columns = self.columns.to_clq();
        let from = self.from.to_cql();

        let mut where_ = String::new();
        if let Some(w) = self.where_.clone() {
            where_ = w.to_cql();
        }

        let mut order_by = String::new();
        if let Some(o) = self.order_by.clone() {
            order_by = o.to_cql()
        }

        let mut limit = String::new();
        if let Some(l) = self.limit.clone() {
            limit = l.to_cql();
        }

        let mut allow_filtering = String::new();
        if self.allow_filtering {
            allow_filtering = String::from("ALLOW FILTERING");
        }

        format!("SELECT {} {} {} {} {} {}", columns, from, where_, order_by, limit, allow_filtering).trim().to_string()
    }
}

impl Limit {
    fn to_cql(&self) -> String {
        let Limit(lm) = self.clone();
        format!("LIMIT {}", lm)
    }
}
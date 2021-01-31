use super::column_expr::Column;

#[derive(Clone)]
pub struct Order {
    pub(crate) column: Column,
    pub(crate) by: OrderDirection
}

#[derive(Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

use OrderDirection::*;

impl Order {
    pub fn to_cql(&self) -> String {
        let Self{column, by} = self.clone();
        let Column(c) = column;
        format!("ORDER BY {} {}", c, by.to_cql())
    }
}

impl OrderDirection {
    fn to_cql(&self) -> String {
        match self {
            Asc  => String::from("ASC"),
            Desc => String::from("DESC"),
        }
    }
}
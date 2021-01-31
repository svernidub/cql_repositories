use std::collections::HashSet;

use super::column_expr::{AnyColumn, Column};

#[derive(Clone)]
pub enum Columns {
    All,
    List(HashSet<AnyColumn>),
    Distinct(HashSet<Column>),
}

use Columns::*;

impl Columns {
    pub fn to_clq(&self) -> String {
        match self {
            All => String::from("ALL"),
            List(xs) => {
                xs.into_iter().map(|x| x.to_cql()).collect::<Vec<String>>().join(", ")
            }
            Distinct(xs) => {
                let columns = xs.into_iter().map(|Column(x)| x.clone()).collect::<Vec<String>>().join(", ");
                format!("DISTINCT {}", columns)
            }
        }
    }
}
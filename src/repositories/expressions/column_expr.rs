#[derive(Clone)]
pub struct Column(pub String);

#[derive(Clone)]
pub struct Alias(pub String);

#[derive(Clone)]
pub enum AnyColumn {
    ColumnName(Column),
    ColumnAlias(Column, Alias)
}

impl AnyColumn {
    pub fn to_cql(&self) -> String {
        match self {
            AnyColumn::ColumnName(Column(c)) => format!("{}", c),
            AnyColumn::ColumnAlias(Column(c), Alias(a)) => format!("{} as {}", c, a)
        }
    }
}

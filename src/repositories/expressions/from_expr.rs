#[derive(Clone)]
pub struct FromExpr {
    pub(crate) keyspace: Option<String>,
    pub(crate) table: String
}

impl FromExpr {
    pub fn to_cql(&self) -> String {
        let Self{ keyspace, table } = self.clone();

        let mut keyspace_str = "".to_string();

        if let Some(k) = keyspace {
            keyspace_str = format!("{}.", k);
        }

        format!("FROM {}{}", keyspace_str, table)
    }
}
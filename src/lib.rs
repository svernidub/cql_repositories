pub mod repositories;
use repositories::expressions::*;
use repositories::expressions::select_expr;

#[cfg(test)]
mod tests {
    use crate::repositories::expressions::select_expr::{SelectExpression, Limit};
    use crate::repositories::expressions::from_expr::FromExpr;
    use crate::repositories::expressions::where_expr::WhereExpr::{WhereExpression, InExpression};
    use crate::repositories::expressions::columns_expr::Columns;
    use crate::repositories::expressions::where_expr::{Where, Operator, InExpr};
    use crate::repositories::expressions::value_expr::Value::{Text, Int};
    use crate::repositories::expressions::column_expr::Column;
    use crate::repositories::expressions::order_expr::{Order, OrderDirection};

    #[test]
    fn it_works() {
        let expr = SelectExpression{
            columns: Columns::All,
            from: FromExpr{ keyspace: Some("test".to_string()), table: "test_table".to_string() },
            where_: Some(Where(vec!(
                WhereExpression(Column("name".to_string()), Operator("=".to_string()), Text("Someone".to_string())),
                InExpression(Column("id".to_string()), InExpr(vec!(Int(1), Int(2), Int(3))))
            ))),
            order_by: Some(Order{ column: Column("id".to_string()), by: OrderDirection::Asc }),
            limit: Some(Limit(10)),
            allow_filtering: true
        };

        println!("{}", expr.to_cql())
    }
}

use super::User;
use njord::condition::{Condition, Value};
use njord::keys::AutoIncrementPrimaryKey;
use njord::oracle;
use std::vec;

#[test]
fn update_row() {
    insert_row();

    let connection_string = "//localhost:1521/FREEPDB1";
    let mut conn = oracle::open("njord_user", "njord_password", connection_string);

    let columns = vec!["username".to_string()];
    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("chasewillden2".to_string()),
    );

    match conn {
        Ok(ref mut c) => {
            let result = oracle::update(User::default())
                .set(columns)
                .where_clause(condition)
                .limit(4)
                .offset(0)
                .build(c);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to UPDATE: {:?}", e);
        }
    }
}

/// Helper function to insert a row to be updated
fn insert_row() {
    let connection_string = "//localhost:1521/FREEPDB1";
    let mut conn = oracle::open("njord_user", "njord_password", connection_string);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "chasewillden2".to_string(),
        email: "chase.willden@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = oracle::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

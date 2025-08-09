// use sea_orm::{Database, DbErr};
// use crate::entity::models; // whatever your generated entity module is
use rusqlite::{params, Connection, Result};

#[tauri::command(rename_all="snake_case")]
pub fn get_models() -> String {
    let conn = match Connection::open("../../data/journal.db") {
        Ok(c) => c,
        Err(e) => return format!("DB open error: {}", e),
    };

    let mut stmt = match conn.prepare("SELECT id, name FROM models") {
        Ok(s) => s,
        Err(e) => return format!("Prepare error: {}", e),
    };

    let model_iter = match stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    }) {
        Ok(iter) => iter,
        Err(e) => return format!("Query error: {}", e),
    };

    for model in model_iter {
        match model {
            Ok((id, name)) => println!("Model {}: {}", id, name),
            Err(e) => return format!("Row error: {}", e),
        }
    }

    "Okay!".to_string()
}


// #[tauri::command]
// #[tokio::main]
// pub async fn main() -> Result<(), DbErr> {
//     // Connect to the SQLite database
//     let db = Database::connect("sqlite://../../data/journal.db?mode=rwc").await?;

//     // Query all rows from the "models" table
//     let all_models: Vec<models::Model> = models::Entity::find()
//         .all(&db)
//         .await?;

//     // Print them
//     for model in all_models {
//         println!("{:?}", model);
//     }
    
//     Ok(())
// }


// #![allow(unused_imports, dead_code)]

// pub mod common;

// pub use sea_orm::{Database, DbConn, entity::*, error::*, query::*, sea_query, tests_cfg::*};

// // cargo test --features sqlx-sqlite,runtime-tokio --test basic
// // export DATABASE_URL=mysql://root:root@localhost:3306
// // export DATABASE_URL=sqlite::memory:
// #[sea_orm_macros::test]
// #[cfg(feature = "sqlx-sqlite")]
// async fn main() -> Result<(), DbErr> {
//     dotenv::from_filename(".env.local").ok();
//     dotenv::from_filename(".env").ok();

//     let base_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_owned());

//     let db: DbConn = Database::connect(&base_url).await?;
//     setup_schema(&db).await?;
//     crud_cake(&db).await?;

//     Ok(())
// }

// #[cfg(feature = "sqlx-sqlite")]
// async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
//     use sea_query::*;

//     let stmt = sea_query::Table::create()
//         .table(cake::Entity)
//         .col(
//             ColumnDef::new(cake::Column::Id)
//                 .integer()
//                 .not_null()
//                 .auto_increment()
//                 .primary_key(),
//         )
//         .col(ColumnDef::new(cake::Column::Name).string())
//         .to_owned();

//     let result = db.execute(&stmt).await?;
//     println!("Create table cake: {result:?}");

//     Ok(())
// }

// #[cfg(feature = "sqlx-sqlite")]
// async fn crud_cake(db: &DbConn) -> Result<(), DbErr> {
//     let apple = cake::ActiveModel {
//         name: Set("Apple Pie".to_owned()),
//         ..Default::default()
//     };

//     let mut apple = apple.save(db).await?;

//     println!();
//     println!("Inserted: {apple:?}");

//     assert_eq!(
//         apple,
//         cake::ActiveModel {
//             id: Unchanged(1),
//             name: Unchanged("Apple Pie".to_owned()),
//         }
//     );

//     apple.name = Set("Lemon Tart".to_owned());

//     let apple = apple.save(db).await?;

//     println!();
//     println!("Updated: {apple:?}");

//     let count = cake::Entity::find().count(db).await?;

//     println!();
//     println!("Count: {count:?}");
//     assert_eq!(count, 1);

//     let apple = cake::Entity::find_by_id(1).one(db).await?;

//     assert_eq!(
//         Some(cake::Model {
//             id: 1,
//             name: "Lemon Tart".to_owned(),
//         }),
//         apple
//     );

//     let apple: cake::Model = apple.unwrap();

//     let result = apple.delete(db).await?;

//     println!();
//     println!("Deleted: {result:?}");

//     let apple = cake::Entity::find_by_id(1).one(db).await?;

//     assert_eq!(None, apple);

//     let count = cake::Entity::find().count(db).await?;

//     println!();
//     println!("Count: {count:?}");
//     assert_eq!(count, 0);

//     Ok(())
// }
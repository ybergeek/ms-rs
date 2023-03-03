use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS product (
                product_id varchar(40) NOT NULL, 
                name varchar(20), 
                description varchar(200), 
                price int, 
                count int, 
                image_url varchar(40), 
                PRIMARY KEY(product_id)
            );
            
            CREATE TABLE IF NOT EXISTS tag (
                tag_id SERIAL, 
                name varchar(20),
              display_name varchar(20), 
                PRIMARY KEY(tag_id)
            );
            
            CREATE TABLE IF NOT EXISTS product_tag (
                product_id varchar(40), 
                tag_id INTEGER NOT NULL, 
                FOREIGN KEY (product_id) 
                    REFERENCES product(product_id), 
                FOREIGN KEY(tag_id)
                    REFERENCES tag(tag_id),
                PRIMARY KEY(product_id,tag_id)
            );"
        )
        .await?;

        // Construct a `Statement` if the SQL contains value bindings
        let stmt = Statement::from_sql_and_values(
            manager.get_database_backend(),
            r#"INSERT INTO `cake` (`name`) VALUES (?)"#,
            ["Cheese Cake".into()]
            r#"INSERT INTO product VALUES(?,?)"#,
            ('6d62d909-f957-430e-8689-b5129c0bb75e', 'Pocket Watch', 'Properly dapper.', 385, 33, '/assets/pocket_watch.jpg');

        );
        db.execute(stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `cake`")
            .await?;

        Ok(())
    }
}
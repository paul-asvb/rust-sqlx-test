use dotenv;
use sqlx::PgPool;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL")?).await?;

    let create_tables = std::include_str!("../migrations/01.sql");

    sqlx::query(create_tables).execute(&pool).await?;

    sqlx::query!(
        r#"
            INSERT INTO rust_test ( id )
            VALUES ( $1 )
        "#,
        1
    )
    .execute(&pool)
    .await?;

    sqlx::query!(
        r#"
            INSERT INTO rust_test ( id )
            VALUES ( $1 )
        "#,
        1
    )
    .execute(&pool)
    .await?;

    let test_struct = sqlx::query_as!(
        TestStruct,
        r#"
        SELECT *
        FROM rust_test
        "#
    )
    .fetch_all(&pool)
    .await?;

    print!("{:?}", test_struct);

    Ok(())
}
#[derive(Debug)]
struct TestStruct {
    id: Option<i32>,
}

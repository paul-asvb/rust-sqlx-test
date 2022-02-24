use sqlx::PgPool;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect("postgres://postgres@localhost:5432").await?;

    let create_tables = std::include_str!("../migrations/20220224.sql");

    sqlx::query(create_tables).execute(&pool).await?;

    // let rec = sqlx::query!(
    //     r#"
    //         INSERT INTO rust_test ( id )
    //         VALUES ( $1 )
    //     "#,
    //     1
    // )
    // .execute(pool)
    // .await?;

    Ok(())
}

struct TestStruct {
    id: Option<u16>,
}

use dotenv;

#[async_std::main]
async fn main() -> anyhow::Result<()> {

    let conn = SqliteConnection::connect("sqlite::memory:").await?;


    sqlx::query(
        r#"
        INSERT INTO rust_test ( id, some_bool, name, current_mood  )
        VALUES (1,false,'blablalba', 'sad')
        "#
    )
    .execute(&conn)
    .await?;

    // let test_struct = sqlx::query_as!(
    //     TestStruct,
    //     r#"
    //     SELECT 
    //     id, some_bool, name, current_mood as "current_mood: Mood"
    //     FROM rust_test
    //     "#
    // )
    // .fetch_all(&pool)
    // .await?;

    //print!("{:?}", test_struct);

    Ok(())
}
#[derive(Debug)]
struct TestStruct {
    id: Option<i32>,
    some_bool: Option<bool>,
    name: Option<String>,
    current_mood: Option<Mood>,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "mood", rename_all = "lowercase")]
enum Mood {
    Sad,
    Ok,
    Happy,
}

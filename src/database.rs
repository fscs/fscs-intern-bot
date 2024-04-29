use poise::serenity_prelude::UserId;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Connection, Executor, Sqlite, SqliteConnection, SqlitePool};
use uuid::Uuid;

use crate::{rest, structs::Person};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub discord_id: i64,
    pub person_id: String,
}

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let db_url = "sqlite://mappings.db";
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let conn = SqlitePool::connect(db_url).await.unwrap();
    match sqlx::migrate!().run(&conn).await {
        Ok(_) => println!("Migrations ran successfully"),
        Err(error) => panic!("error: {}", error),
    }

    Ok(conn)
}

pub async fn get_name(conn: SqlitePool, discord_id: UserId) -> Result<Person, sqlx::Error> {
    let userid = i64::from(discord_id);
    let name = sqlx::query_as::<_, User>("SELECT * FROM users WHERE discord_id = $1")
        .bind(userid)
        .fetch_one(&conn)
        .await?;

    let persons = rest::get_persons().await;

    let person = persons
        .iter()
        .find(|person| person.id.to_string() == name.person_id)
        .unwrap();

    Ok(person.clone())
}

pub async fn map_antrag_thread(
    conn: SqlitePool,
    antrag_id: Uuid,
    thread_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO antragsthreads (thread_id, antrag_id) VALUES ($1, $2)")
        .bind(thread_id)
        .bind(antrag_id.to_string())
        .execute(&conn)
        .await?;
    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct antragsid {
    antrag_id: String,
    thread_id: i64,
}

pub async fn get_antrag_thread(
    conn: SqlitePool,
    thread_id: i64,
) -> Result<Option<Uuid>, sqlx::Error> {
    let antrag =
        sqlx::query_as::<_, antragsid>("SELECT * FROM antragsthreads WHERE thread_id = $1")
            .bind(thread_id)
            .fetch_one(&conn)
            .await
            .unwrap();

    Ok(Some(Uuid::parse_str(&antrag.antrag_id).unwrap()))
}

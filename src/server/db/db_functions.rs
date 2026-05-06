use bcrypt::{DEFAULT_COST, hash, verify};

/// 1. Initialize the Database Connection
use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    // Read the database URL from the environment (.env file)
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in your environment");

    println!("{:?}", database_url);

    // Configure and establish the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10) // Set the maximum number of connections
        .min_connections(2)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn sign_up(
    pool: &PgPool,
    username: &str,
    password: &str,
    student_id: &str,
) -> Result<String, String> {
    let hashed_pw = hash(password, DEFAULT_COST).map_err(|e| format!("Hashing failed: {}", e))?;

    let row = sqlx::query!(
        "INSERT INTO users (student_id, name, password) VALUES ($1, $2, $3)",
        student_id,
        username,
        hashed_pw
    )
    .execute(pool)
    .await
    .map_err(|e| format!("Database error (maybe username taken?): {}", e));

    let row = row.unwrap();
}

/// 3. Sign In (Login)
pub async fn sign_in(pool: &PgPool, username: &str, password: &str) -> Result<String, String> {
    let row_opt = sqlx::query!(
        "SELECT id::text, password FROM users WHERE name = $1",
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Query failed: {}", e))?;

    match row_opt {
        Some(row) => {
            let id: String = row.get(0);
            let stored_hash: String = row.get(1);

            let is_valid = verify(password, &stored_hash).unwrap_or(false);

            if is_valid {
                Ok(id)
            } else {
                Err("Invalid password".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
}

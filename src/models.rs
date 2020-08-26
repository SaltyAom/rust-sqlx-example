use serde::{Deserialize, Serialize};
use sqlx::{ FromRow, PgPool };
use anyhow::Result;

// Database's table struct
#[derive(FromRow, Serialize)]
pub struct Team {
    pub name: String,
    pub role: String,
    pub age: i32,
}

// Struct receive from POST form
#[derive(Deserialize)]
pub struct Member {
    pub name: String,
    pub role: String,
    pub age: i32,
}

impl Team {
    pub async fn new_member(member: &Member, connection: &PgPool) -> Result<()> {
        sqlx::query!(r#"
            INSERT INTO team(name, role, age) VALUES($1, $2, $3)
        "#, member.name, member.role, member.age)
            .execute(connection)
            .await?;

        Ok(())
    }

    pub async fn list(connection: &PgPool) -> Result<Vec<Team>> {
        let members = sqlx::query_as::<_, Team>(r#"
            SELECT name, role, age FROM team
        "#)
            .fetch_all(connection)
            .await?;

        Ok(
            members
        )
    }

    pub async fn specific(connection: &PgPool) -> Result<Team> {
        let member = sqlx::query_as::<_, Team>(r#"
            SELECT name, role, age FROM team WHERE name = $1
        "#)
            .bind("SaltyAom")
            .fetch_one(connection)
            .await?;

        Ok(
            member
        )
    }
}

#[derive(Serialize)]
pub struct ApiResult {
    pub success: bool,
}

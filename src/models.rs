use serde::{Deserialize, Serialize};
use sqlx::{ FromRow, PgPool };
use anyhow::Result;

#[derive(FromRow)]
pub struct Team {
    pub name: String,
    pub role: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize)]
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

    pub async fn list(connection: &PgPool) -> Result<Vec<Member>> {
        let mut member_list: Vec<Member> = vec![];

        let members = sqlx::query!(r#"
            SELECT name, role, age FROM team
        "#)
            .fetch_all(connection)
            .await?;

        for member in members {
            member_list.push(Member {
                name: member.name,
                role: member.role,
                age: member.age
            })
        }

        Ok(
            member_list
        )
    }

    pub async fn specific(connection: &PgPool) -> Result<Member> {
        let member = sqlx::query!(r#"
            SELECT name, role, age FROM team WHERE name = $1
        "#, "SaltyAom")
            .fetch_one(connection)
            .await?;

        Ok(
            Member {
                name: member.name,
                role: member.role,
                age: member.age,
            }
        )
    }
}

#[derive(Serialize)]
pub struct ApiResult {
    pub success: bool,
}

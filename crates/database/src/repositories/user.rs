use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::Utc;
use crate::entities::user;

pub async fn create_user_if_not_exist(
    db: &DatabaseConnection,
    id: i64,
    username: Option<String>,
    first_name: Option<String>,
) -> Result<i64, DbErr> {
    let model = user::ActiveModel{
        id: Set(id),
        username: Set(username),
        first_name: Set(first_name),
        created_at: Set(DateTimeWithTimeZone::from(Utc::now())),
    };

    let user = user::Entity::find_by_id(id)
        .one(db)
        .await?;


    let id = if let Some(user) = user {
        user.id
    } else {
        user::Entity::insert(model).exec(db).await?.last_insert_id
    };

    Ok(id)
}

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}
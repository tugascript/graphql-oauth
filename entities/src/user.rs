use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "String(Some(250))", unique)]
    pub email: String,
    #[sea_orm(column_type = "String(Some(50))")]
    pub first_name: String,
    #[sea_orm(column_type = "String(Some(50))")]
    pub last_name: String,
    #[sea_orm(default_value = false)]
    pub confirmed: bool,
    #[sea_orm(default_value = false)]
    pub two_factor_enabled: bool,
    #[sea_orm(default_value = 1)]
    pub version: i16,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        let current_time = Utc::now().naive_utc();
        self.updated_at = ActiveValue::Set(current_time);
        if insert {
            self.created_at = ActiveValue::Set(current_time);
        }
        Ok(self)
    }
}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog")]
pub struct Blog {
    #[sea_orm(primary_key)]
    pub id: i32,
}
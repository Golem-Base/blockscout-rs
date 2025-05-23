//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "address_coin_balances")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "VarBinary(StringLen::None)"
    )]
    pub address_hash: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub block_number: i64,
    #[sea_orm(column_type = "Decimal(Some((100, 0)))", nullable)]
    pub value: Option<Decimal>,
    pub value_fetched_at: Option<DateTime>,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "account_identities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
    pub plan_id: Option<i64>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)", nullable)]
    pub uid: Option<Vec<u8>>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)", nullable, unique)]
    pub uid_hash: Option<Vec<u8>>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)", nullable)]
    pub email: Option<Vec<u8>>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)", nullable)]
    pub avatar: Option<Vec<u8>>,
    pub verification_email_sent_at: Option<DateTime>,
    pub otp_sent_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::account_api_keys::Entity")]
    AccountApiKeys,
    #[sea_orm(
        belongs_to = "super::account_api_plans::Entity",
        from = "Column::PlanId",
        to = "super::account_api_plans::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AccountApiPlans,
    #[sea_orm(has_many = "super::account_custom_abis::Entity")]
    AccountCustomAbis,
    #[sea_orm(has_many = "super::account_public_tags_requests::Entity")]
    AccountPublicTagsRequests,
    #[sea_orm(has_many = "super::account_tag_addresses::Entity")]
    AccountTagAddresses,
    #[sea_orm(has_many = "super::account_tag_transactions::Entity")]
    AccountTagTransactions,
    #[sea_orm(has_many = "super::account_watchlists::Entity")]
    AccountWatchlists,
}

impl Related<super::account_api_keys::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountApiKeys.def()
    }
}

impl Related<super::account_api_plans::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountApiPlans.def()
    }
}

impl Related<super::account_custom_abis::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountCustomAbis.def()
    }
}

impl Related<super::account_public_tags_requests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountPublicTagsRequests.def()
    }
}

impl Related<super::account_tag_addresses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountTagAddresses.def()
    }
}

impl Related<super::account_tag_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountTagTransactions.def()
    }
}

impl Related<super::account_watchlists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountWatchlists.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

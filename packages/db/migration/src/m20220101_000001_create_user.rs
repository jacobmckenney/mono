use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Name).string())
                    .col(ColumnDef::new(User::Image).string())
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserId).string().not_null())
                    .col(ColumnDef::new(Session::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::ExpiresAt).timestamp().not_null())
                    .col(ColumnDef::new(Session::DeviceIp).string())
                    .col(ColumnDef::new(Session::UserAgent).string())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
            .unwrap();
        return manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await;
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Image,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    CreatedAt,
    ExpiresAt,
    DeviceIp,
    UserAgent,
}

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CashFlow::Table)
                    .if_not_exists()
                    .col(pk_auto(CashFlow::Id))
                    .col(string(CashFlow::Amount))
                    .col(string(CashFlow::Name))
                    .col(string(CashFlow::Flow))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CashFlow::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CashFlow {
    Table,
    Id,
    Amount,
    Name,
    Flow,
}

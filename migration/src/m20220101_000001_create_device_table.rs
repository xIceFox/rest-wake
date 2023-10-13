use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Device)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Name)
                            .string_len(30)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Mac)
                        .char_len(17)
                        .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Post::Device).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Device,
    Name,
    Mac,
}

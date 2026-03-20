use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Persons::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Persons::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Persons::Name).string().not_null())
                    .col(ColumnDef::new(Persons::Notes).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Mobiles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Mobiles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Mobiles::PersonId).integer().not_null())
                    .col(ColumnDef::new(Mobiles::Number).string().not_null())
                    .col(ColumnDef::new(Mobiles::Label).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-mobiles-person-id")
                            .from(Mobiles::Table, Mobiles::PersonId)
                            .to(Persons::Table, Persons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Emails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Emails::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Emails::PersonId).integer().not_null())
                    .col(ColumnDef::new(Emails::Address).string().not_null())
                    .col(ColumnDef::new(Emails::Label).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-emails-person-id")
                            .from(Emails::Table, Emails::PersonId)
                            .to(Persons::Table, Persons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Emails::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Mobiles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Persons::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Persons {
    Table,
    Id,
    Name,
    Notes,
}

#[derive(DeriveIden)]
enum Mobiles {
    Table,
    Id,
    PersonId,
    Number,
    Label,
}

#[derive(DeriveIden)]
enum Emails {
    Table,
    Id,
    PersonId,
    Address,
    Label,
}

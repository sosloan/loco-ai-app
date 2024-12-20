use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create agents table
        manager
            .create_table(
                Table::create()
                    .table(Agents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Agents::Id).uuid().primary_key())
                    .col(ColumnDef::new(Agents::Name).string().not_null())
                    .col(ColumnDef::new(Agents::Description).text())
                    .col(ColumnDef::new(Agents::Type).string().not_null())
                    .col(ColumnDef::new(Agents::Status).string().not_null())
                    .col(ColumnDef::new(Agents::Configuration).json())
                    .col(ColumnDef::new(Agents::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Agents::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create agent_capabilities table
        manager
            .create_table(
                Table::create()
                    .table(AgentCapabilities::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AgentCapabilities::Id).uuid().primary_key())
                    .col(ColumnDef::new(AgentCapabilities::AgentId).uuid().not_null())
                    .col(ColumnDef::new(AgentCapabilities::Name).string().not_null())
                    .col(ColumnDef::new(AgentCapabilities::Description).text())
                    .col(ColumnDef::new(AgentCapabilities::Parameters).json())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_agent_capabilities_agent")
                            .from(AgentCapabilities::Table, AgentCapabilities::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentCapabilities::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Agents::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Agents {
    Table,
    Id,
    Name,
    Description,
    Type,
    Status,
    Configuration,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum AgentCapabilities {
    Table,
    Id,
    AgentId,
    Name,
    Description,
    Parameters,
}

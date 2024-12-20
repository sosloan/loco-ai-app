use sea_orm_migration::prelude::*;
use crate::m20231220_000001_agents::Agents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create tasks table
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tasks::Id).uuid().primary_key())
                    .col(ColumnDef::new(Tasks::AgentId).uuid().not_null())
                    .col(ColumnDef::new(Tasks::Name).string().not_null())
                    .col(ColumnDef::new(Tasks::Description).text())
                    .col(ColumnDef::new(Tasks::Status).string().not_null())
                    .col(ColumnDef::new(Tasks::Priority).integer().not_null())
                    .col(ColumnDef::new(Tasks::Input).json())
                    .col(ColumnDef::new(Tasks::Output).json())
                    .col(ColumnDef::new(Tasks::Metadata).json())
                    .col(ColumnDef::new(Tasks::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Tasks::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Tasks::CompletedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tasks_agent")
                            .from(Tasks::Table, Tasks::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create task_dependencies table
        manager
            .create_table(
                Table::create()
                    .table(TaskDependencies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TaskDependencies::Id).uuid().primary_key())
                    .col(ColumnDef::new(TaskDependencies::TaskId).uuid().not_null())
                    .col(ColumnDef::new(TaskDependencies::DependsOnTaskId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_dependencies_task")
                            .from(TaskDependencies::Table, TaskDependencies::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_task_dependencies_depends_on")
                            .from(TaskDependencies::Table, TaskDependencies::DependsOnTaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskDependencies::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Tasks {
    Table,
    Id,
    AgentId,
    Name,
    Description,
    Status,
    Priority,
    Input,
    Output,
    Metadata,
    CreatedAt,
    UpdatedAt,
    CompletedAt,
}

#[derive(Iden)]
pub enum TaskDependencies {
    Table,
    Id,
    TaskId,
    DependsOnTaskId,
}

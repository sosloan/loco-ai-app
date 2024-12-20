use sea_orm_migration::prelude::*;
use crate::m20231220_000001_agents::Agents;
use crate::m20220101_000001_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create conversations table
        manager
            .create_table(
                Table::create()
                    .table(Conversations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Conversations::Id).uuid().primary_key())
                    .col(ColumnDef::new(Conversations::AgentId).uuid().not_null())
                    .col(ColumnDef::new(Conversations::UserId).uuid().not_null())
                    .col(ColumnDef::new(Conversations::Title).string())
                    .col(ColumnDef::new(Conversations::Status).string().not_null())
                    .col(ColumnDef::new(Conversations::Metadata).json())
                    .col(ColumnDef::new(Conversations::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Conversations::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_conversations_agent")
                            .from(Conversations::Table, Conversations::AgentId)
                            .to(Agents::Table, Agents::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_conversations_user")
                            .from(Conversations::Table, Conversations::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create messages table
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Messages::Id).uuid().primary_key())
                    .col(ColumnDef::new(Messages::ConversationId).uuid().not_null())
                    .col(ColumnDef::new(Messages::Role).string().not_null())
                    .col(ColumnDef::new(Messages::Content).text().not_null())
                    .col(ColumnDef::new(Messages::Metadata).json())
                    .col(ColumnDef::new(Messages::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_conversation")
                            .from(Messages::Table, Messages::ConversationId)
                            .to(Conversations::Table, Conversations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create memories table (for long-term storage)
        manager
            .create_table(
                Table::create()
                    .table(Memories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Memories::Id).uuid().primary_key())
                    .col(ColumnDef::new(Memories::AgentId).uuid().not_null())
                    .col(ColumnDef::new(Memories::Type).string().not_null())
                    .col(ColumnDef::new(Memories::Content).text().not_null())
                    .col(ColumnDef::new(Memories::Embedding).binary())
                    .col(ColumnDef::new(Memories::Metadata).json())
                    .col(ColumnDef::new(Memories::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Memories::LastAccessed).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_memories_agent")
                            .from(Memories::Table, Memories::AgentId)
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
            .drop_table(Table::drop().table(Memories::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Conversations::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Conversations {
    Table,
    Id,
    AgentId,
    UserId,
    Title,
    Status,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Messages {
    Table,
    Id,
    ConversationId,
    Role,
    Content,
    Metadata,
    CreatedAt,
}

#[derive(Iden)]
pub enum Memories {
    Table,
    Id,
    AgentId,
    Type,
    Content,
    Embedding,
    Metadata,
    CreatedAt,
    LastAccessed,
}

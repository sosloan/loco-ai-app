use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create knowledge_base table
        manager
            .create_table(
                Table::create()
                    .table(KnowledgeBase::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(KnowledgeBase::Id).uuid().primary_key())
                    .col(ColumnDef::new(KnowledgeBase::Name).string().not_null())
                    .col(ColumnDef::new(KnowledgeBase::Description).text())
                    .col(ColumnDef::new(KnowledgeBase::Type).string().not_null())
                    .col(ColumnDef::new(KnowledgeBase::Configuration).json())
                    .col(ColumnDef::new(KnowledgeBase::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(KnowledgeBase::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create knowledge_items table
        manager
            .create_table(
                Table::create()
                    .table(KnowledgeItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(KnowledgeItems::Id).uuid().primary_key())
                    .col(ColumnDef::new(KnowledgeItems::KnowledgeBaseId).uuid().not_null())
                    .col(ColumnDef::new(KnowledgeItems::Type).string().not_null())
                    .col(ColumnDef::new(KnowledgeItems::Content).text().not_null())
                    .col(ColumnDef::new(KnowledgeItems::Embedding).binary())
                    .col(ColumnDef::new(KnowledgeItems::Metadata).json())
                    .col(ColumnDef::new(KnowledgeItems::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(KnowledgeItems::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_knowledge_items_base")
                            .from(KnowledgeItems::Table, KnowledgeItems::KnowledgeBaseId)
                            .to(KnowledgeBase::Table, KnowledgeBase::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create learning_models table (inspired by DSPy)
        manager
            .create_table(
                Table::create()
                    .table(LearningModels::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(LearningModels::Id).uuid().primary_key())
                    .col(ColumnDef::new(LearningModels::Name).string().not_null())
                    .col(ColumnDef::new(LearningModels::Type).string().not_null())
                    .col(ColumnDef::new(LearningModels::Version).string().not_null())
                    .col(ColumnDef::new(LearningModels::Configuration).json())
                    .col(ColumnDef::new(LearningModels::Metrics).json())
                    .col(ColumnDef::new(LearningModels::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(LearningModels::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(LearningModels::LastTrainedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        // Create model_training_data table
        manager
            .create_table(
                Table::create()
                    .table(ModelTrainingData::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ModelTrainingData::Id).uuid().primary_key())
                    .col(ColumnDef::new(ModelTrainingData::ModelId).uuid().not_null())
                    .col(ColumnDef::new(ModelTrainingData::Input).json().not_null())
                    .col(ColumnDef::new(ModelTrainingData::Output).json().not_null())
                    .col(ColumnDef::new(ModelTrainingData::Metadata).json())
                    .col(ColumnDef::new(ModelTrainingData::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_data_model")
                            .from(ModelTrainingData::Table, ModelTrainingData::ModelId)
                            .to(LearningModels::Table, LearningModels::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModelTrainingData::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LearningModels::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(KnowledgeItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(KnowledgeBase::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum KnowledgeBase {
    Table,
    Id,
    Name,
    Description,
    Type,
    Configuration,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum KnowledgeItems {
    Table,
    Id,
    KnowledgeBaseId,
    Type,
    Content,
    Embedding,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum LearningModels {
    Table,
    Id,
    Name,
    Type,
    Version,
    Configuration,
    Metrics,
    CreatedAt,
    UpdatedAt,
    LastTrainedAt,
}

#[derive(Iden)]
pub enum ModelTrainingData {
    Table,
    Id,
    ModelId,
    Input,
    Output,
    Metadata,
    CreatedAt,
}

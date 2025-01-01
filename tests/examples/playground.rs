use loco_rs::prelude::*;
use myapp::app::App;
use sea_orm::ActiveModelTrait;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_playground_examples() -> loco_rs::Result<()> {
    let ctx = loco_rs::testing::boot_test::<App>().await?;

    let active_model: articles::ActiveModel = articles::ActiveModel {
        title: Set(Some("how to build apps in 3 steps".to_string())),
        content: Set(Some("use Loco: https://loco.rs".to_string())),
        ..Default::default()
    };
    active_model.insert(&ctx.db).await.unwrap();

    let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    assert!(!res.is_empty());

    Ok(())
}

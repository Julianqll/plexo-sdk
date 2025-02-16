use std::{error::Error, str::FromStr, sync::Arc};

use chrono::Local;
use dotenv::dotenv;

use plexo_sdk::{
    backend::{
        engine::{SDKConfig, SDKEngine},
        loaders::SDKLoaders,
    },
    resources::{
        projects::{
            operations::{GetProjectsInputBuilder, GetProjectsWhereBuilder, ProjectCrudOperations},
            relations::ProjectRelations,
        },
        tasks::{
            extensions::{CreateTasksInputBuilder, TasksExtensionOperations},
            operations::{CreateTaskInputBuilder, TaskCrudOperations},
            relations::TaskRelations,
            task::TaskStatus,
        },
    },
};

use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let engine = SDKEngine::new(SDKConfig::from_env()).await?;
    let engine = Arc::new(engine);

    let loaders = SDKLoaders::new(engine.clone());

    let projects = engine
        .get_projects(
            GetProjectsInputBuilder::default()
                .filter(
                    GetProjectsWhereBuilder::default()
                        .ids(vec![
                            Uuid::from_str("21c87de9-5ae5-4fca-ad41-ed8bc02c7955")?,
                            Uuid::from_str("69e9b0ee-603a-407b-8a2d-99033de6ae86")?,
                            Uuid::from_str("0d6b949f-b64a-4aca-a6f6-b79fc8e6228e")?,
                        ])
                        .name("Plexo".to_string())
                        .build()?,
                )
                .limit(1_000_000)
                .build()?,
        )
        .await?;

    println!("projects: {:?}", projects);

    let project = projects.first().unwrap();

    let project_owner = project.owner(&loaders).await?;

    println!("project owner: {:?}", project_owner.name);

    let tasks = engine.get_tasks(None).await?;
    let task = tasks.first().unwrap();

    let task_owner = task.owner(&loaders).await?;

    println!("task owner: {:?}", task_owner.name);

    let tasks = engine
        .create_tasks(
            CreateTasksInputBuilder::default()
                .tasks(vec![
                    CreateTaskInputBuilder::default()
                        .title("Example Task".to_string())
                        .owner_id(task_owner.id)
                        .subtasks(vec![
                            CreateTaskInputBuilder::default()
                                .title("Example Task 1".to_string())
                                .owner_id(task_owner.id)
                                .build()?,
                            CreateTaskInputBuilder::default()
                                .title("Example Task 2".to_string())
                                .owner_id(task_owner.id)
                                .build()?,
                        ])
                        .build()?,
                    CreateTaskInputBuilder::default()
                        .title("Real Task".to_string())
                        .status(TaskStatus::Done)
                        .due_date(Local::now().into())
                        .owner_id(task_owner.id)
                        .build()?,
                ])
                .build()?,
        )
        .await
        .unwrap();

    println!("\ncreated tasks: {:?}", tasks);

    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(())
}

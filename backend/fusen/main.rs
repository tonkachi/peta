use anyhow::Result;
use infrastructure::grpc::Service;
use infrastructure::postgres::DbPool;
use infrastructure::postgres::FusenRepository;
use infrastructure::ulid::IdRepository;
use interface::controller::FusenController;
use std::env;
use usecase::interactor::CreateFusenInteractor;
use usecase::interactor::DeleteFusenInteractor;
use usecase::interactor::GetFusenInteractor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connections = DbPool::new(&database_url);

    let id_repository = IdRepository::default();
    let fusen_repository = FusenRepository::new(connections.clone());
    let create = CreateFusenInteractor::new(id_repository, fusen_repository.clone());
    let get = GetFusenInteractor::new(fusen_repository.clone());
    let delete = DeleteFusenInteractor::new(fusen_repository.clone());
    let controller = FusenController::new(create, get, delete);
    let service = Service::new(controller);

    let addr = "0.0.0.0:50051".parse()?;

    println!("service listening on {}", addr);

    connections.init()?;
    service.serve(addr).await?;

    Ok(())
}

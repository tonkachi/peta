use anyhow::Result;
use infrastructure::grpc::Service;
use infrastructure::postgres::FusenRepository;
use infrastructure::ulid::IdRepository;
use interface::controller::FusenController;
use usecase::interactor::CreateFusenInteractor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id_repository = IdRepository::default();
    let fusen_repository = FusenRepository::default();
    let create = CreateFusenInteractor::new(id_repository, fusen_repository);
    let controller = FusenController::new(create);
    let service = Service::new(controller);

    let addr = "0.0.0.0:50051".parse()?;

    println!("service listening on {}", addr);

    service.serve(addr).await?;

    Ok(())
}

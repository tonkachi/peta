use crate::peta_fusen_v1::Fusen as PBFusen;
use crate::peta_fusen_v1::{CreateRequest, CreateResponse};
use crate::peta_fusen_v1::{GetRequest, GetResponse};
use anyhow::Result;
use derive_new::new;
use tonic::{Request, Response, Status};
use usecase::port::Port;
use usecase::port::*;

pub trait Controller {
    fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status>;
    fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status>;
}

#[derive(new)]
pub struct FusenController<Create, Get>
where
    Create: Port<CreateFusenInputData, CreateFusenOutputData>,
    Get: Port<GetFusenInputData, GetFusenOutputData>,
{
    create_fusen: Create,
    get_fusen: Get,
}

impl<Create, Get> Controller for FusenController<Create, Get>
where
    Create: Port<CreateFusenInputData, CreateFusenOutputData>,
    Get: Port<GetFusenInputData, GetFusenOutputData>,
{
    fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        let input = CreateFusenInputData::new(
            request.get_ref().title.to_string(),
            request.get_ref().note.to_string(),
        );

        match self.create_fusen.handle(input) {
            Ok(output) => Ok(Response::new(CreateResponse {
                fusen: Some(PBFusen {
                    id: output.fusen.id().to_string(),
                    title: output.fusen.title().to_string(),
                    note: output.fusen.note().to_string(),
                }),
            })),
            Err(_) => Err(Status::internal("error")),
        }
    }

    fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let input = GetFusenInputData::new(request.get_ref().id.to_string());

        match self.get_fusen.handle(input) {
            Ok(output) => Ok(Response::new(GetResponse {
                fusen: Some(PBFusen {
                    id: output.fusen.id().to_string(),
                    title: output.fusen.title().to_string(),
                    note: output.fusen.note().to_string(),
                }),
            })),
            Err(_) => Err(Status::internal("error")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use domain::entity::{Fusen, FusenBuilder};
    use domain::vo::{FusenNote, FusenTitle, Id};
    use usecase::port::MockPort;

    fn new_fusen() -> Fusen {
        FusenBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
            .title(
                "Clean Architecture using Rust"
                    .parse::<FusenTitle>()
                    .unwrap(),
            )
            .note(
                "クリーンアーキテクチャをRustで実装してみました〜！"
                    .parse::<FusenNote>()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn test_create_fusen_handle_ok() {
        let entity = new_fusen();

        let mut create = MockPort::<CreateFusenInputData, CreateFusenOutputData>::new();
        let mut get = MockPort::<GetFusenInputData, GetFusenOutputData>::new();
        create
            .expect_handle()
            .returning(|_| Ok(CreateFusenOutputData::new(new_fusen())));
        get.expect_handle()
            .returning(|_| Ok(GetFusenOutputData::new(new_fusen())));
        let sut = FusenController::new(create, get);

        assert_eq!(
            sut.create(Request::new(CreateRequest {
                title: entity.title().to_string(),
                note: entity.note().to_string(),
            }))
            .unwrap()
            .get_ref(),
            Response::new(CreateResponse {
                fusen: Some(PBFusen {
                    id: entity.id().to_string(),
                    title: entity.title().to_string(),
                    note: entity.note().to_string(),
                }),
            })
            .get_ref(),
        );
    }

    #[test]
    fn test_create_fusen_handle_err() {
        let entity = new_fusen();

        let mut create = MockPort::<CreateFusenInputData, CreateFusenOutputData>::new();
        let mut get = MockPort::<GetFusenInputData, GetFusenOutputData>::new();
        create.expect_handle().returning(|_| bail!("error"));
        get.expect_handle().returning(|_| bail!("error"));
        let sut = FusenController::new(create, get);

        assert!(sut
            .create(Request::new(CreateRequest {
                title: entity.title().to_string(),
                note: entity.note().to_string(),
            }))
            .is_err());
    }

    #[test]
    fn test_get_fusen_handle_ok() {
        let entity = new_fusen();

        let mut create = MockPort::<CreateFusenInputData, CreateFusenOutputData>::new();
        let mut get = MockPort::<GetFusenInputData, GetFusenOutputData>::new();
        create
            .expect_handle()
            .returning(|_| Ok(CreateFusenOutputData::new(new_fusen())));
        get.expect_handle()
            .returning(|_| Ok(GetFusenOutputData::new(new_fusen())));
        let sut = FusenController::new(create, get);

        assert_eq!(
            sut.get(Request::new(GetRequest {
                id: entity.id().to_string(),
            }))
            .unwrap()
            .get_ref(),
            Response::new(GetResponse {
                fusen: Some(PBFusen {
                    id: entity.id().to_string(),
                    title: entity.title().to_string(),
                    note: entity.note().to_string(),
                }),
            })
            .get_ref(),
        );
    }

    #[test]
    fn test_get_fusen_handle_err() {
        let entity = new_fusen();

        let mut create = MockPort::<CreateFusenInputData, CreateFusenOutputData>::new();
        let mut get = MockPort::<GetFusenInputData, GetFusenOutputData>::new();
        create.expect_handle().returning(|_| bail!("error"));
        get.expect_handle().returning(|_| bail!("error"));
        let sut = FusenController::new(create, get);

        assert!(sut
            .get(Request::new(GetRequest {
                id: entity.id().to_string()
            }))
            .is_err());
    }
}

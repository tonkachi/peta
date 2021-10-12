use crate::peta_fusen_v1::Fusen as PBFusen;
use crate::peta_fusen_v1::{CreateRequest, CreateResponse};
use anyhow::Result;
use derive_new::new;
use tonic::{Request, Response, Status};
use usecase::port::Port;
use usecase::port::{CreateFusenInputData, CreateFusenOutputData};

pub trait Controller {
    fn create(&self, request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status>;
}

#[derive(new)]
pub struct FusenController<Create>
where
    Create: Port<CreateFusenInputData, CreateFusenOutputData>,
{
    create_fusen: Create,
}

impl<Create> Controller for FusenController<Create>
where
    Create: Port<CreateFusenInputData, CreateFusenOutputData>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use domain::entity::{Fusen, FusenBuilder};
    use domain::vo::{FusenNote, FusenTitle, Id};
    use usecase::port::MockPort;

    #[test]
    fn test_create_fusen_handle_ok() {
        let mut interactor = MockPort::<CreateFusenInputData, CreateFusenOutputData>::new();
        interactor.expect_handle().returning(|input| {
            Ok(CreateFusenOutputData::new(
                FusenBuilder::default()
                    .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Fusen>>().unwrap())
                    .title(input.title.parse::<FusenTitle>().unwrap())
                    .note(input.note.parse::<FusenNote>().unwrap())
                    .build()
                    .unwrap(),
            ))
        });
        let sut = FusenController::new(interactor);

        assert_eq!(
            sut.create(Request::new(CreateRequest {
                title: "title".to_string(),
                note: "note".to_string()
            }))
            .unwrap()
            .get_ref(),
            Response::new(CreateResponse {
                fusen: Some(PBFusen {
                    id: "01F8MECHZX3TBDSZ7XRADM79XE".to_string(),
                    title: "title".to_string(),
                    note: "note".to_string(),
                }),
            })
            .get_ref(),
        );
    }

    #[test]
    fn test_create_fusen_handle_err() {
        let mut interactor = MockPort::<CreateFusenInputData, CreateFusenOutputData>::new();
        interactor.expect_handle().returning(|_| bail!("error"));
        let sut = FusenController::new(interactor);

        assert!(sut
            .create(Request::new(CreateRequest {
                title: "title".to_string(),
                note: "note".to_string()
            }))
            .is_err());
    }
}

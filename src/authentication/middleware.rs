use actix_web_lab::middleware::Next;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::FromRequest;
use actix_web::error::InternalError;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(_) => next.call(req).await,
        None => {
            // User not logged in, they must log in.
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user is not logged in.");
            Err(InternalError::from_response(e, response).into())
        }
    }
}

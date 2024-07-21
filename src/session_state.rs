use std::future::{ready, Ready};

use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::FromRequest;
use uuid::Uuid;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }

    /// Logs out currently logged in user (session.purge()).
    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSession {
    // Return the same error returned by FromRequest for Session.
    type Error = <Session as FromRequest>::Error;
    // Rust does not yet support async syntax in traits.
    // From request expects a future as a return type to allow for extractors with async
    // operations. We don't have any IO so we wrap TypedSession in an always Ready thingie.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}

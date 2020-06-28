use actix_web::{
    dev::Payload,
    web::{HttpRequest, HttpResponse},
    Error,
    FromRequest,
};
use futures::future::{ok, err, Ready};

use crate::home::Claims;
use crate::utils;

#[derive(Debug)]
pub struct UserIdentity{
    pub claims: Claims,
}

/// Extractor for pulling the identity out of a request.
///
/// Simply add "user: AuthUser" to a handler to invoke this.
impl FromRequest for UserIdentity {
    type Error = Error;
    type Config = ();
    type Future = Ready<Result<UserIdentity, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token_str=req.headers().get("Authorization")
                        .and_then(|s|s.to_str().ok()).unwrap();
        match utils::decode_token(token_str){
            Ok(claims)=>{
                ok(UserIdentity{
                    claims:claims,
                })
            },
            Err(e)=>{
                err(HttpResponse::Unauthorized().into())
            }
        }
    }
}

use std::pin::Pin;
use std::task::{Context, Poll};
use std::env::var;
use actix_service::{Service, Transform};
use actix_web::{
    HttpMessage,
    dev::{ServiceRequest,ServiceResponse}, 
    Error,
    // http::{HeaderValue,header}
};
use futures::future::{ok, Ready};
use futures::Future;
use futures::executor::block_on;
use serde_json::{Value as JsonValue};

use crate::utils::{
    error::ApiError,
    auth::decode_with_user_role
};

#[derive(Clone,Copy)]
pub enum AuthType{
    JWT(&'static str),
    APIKEY
}


pub struct Auth{
    pub classification: AuthType
}

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service,classification:self.classification })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    classification:AuthType
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        match self.classification{
            AuthType::JWT(role)=>{
                let auth_cookie = req.cookie("Authorization");

                if auth_cookie == None{
                    return Box::pin(async {
                        Ok(req.error_response(ApiError::Unauthorized("not logged in".to_string())))
                    });
                }

                let auth_cookie_unwrapped = auth_cookie.unwrap();

                let jwt_token = auth_cookie_unwrapped.value();

                let decoded = block_on(async{
                    decode_with_user_role(role,jwt_token,&req.app_data::<crate::AppState>().unwrap()).await
                });

                if let Err(error) = decoded{
                    return Box::pin(async {
                        Ok(req.error_response(error))
                    });
                }

                req.extensions_mut().insert::<JsonValue>(serde_json::from_str(
                    &format!("{{ \"id\":{} }}",decoded.unwrap().id)
                ).unwrap());
            }
            AuthType::APIKEY=>{
                if let Some(key) = req.headers().get("x-api-key"){
                    if key.to_str().unwrap() != var("API_KEY").unwrap() { 
                        return Box::pin(async {
                            Ok(req.error_response(ApiError::Unauthorized("wrong auth".to_string())))
                        });
                    }
                }
                else{
                    return Box::pin(async {
                        Ok(req.error_response(ApiError::Unauthorized("wrong auth".to_string())))
                    });
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
// use crate::todo::{Todo, TodoRequest};
use crate::middlewares::authorize::UserIdentity;
use actix_web::{HttpRequest,delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

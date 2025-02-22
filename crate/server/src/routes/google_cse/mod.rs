use std::sync::Arc;

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace};

use crate::{error::KmsError, result::KResult, KMSServer};

mod jwt;
pub mod operations;

pub use jwt::{jwt_authorization_config, list_jwks_uri, GoogleCseConfig};

/// Error reply for Google CSE
///
/// see: <https://developers.google.com/workspace/cse/reference/structured-errors?hl=en>
#[derive(Serialize, Debug)]
struct CseErrorReply {
    code: u16,
    message: String,
    details: String,
}

impl CseErrorReply {
    fn from(e: KmsError) -> Self {
        Self {
            code: e.status_code().as_u16(),
            message: "A CSE request to the Cosmian KMS failed".to_string(),
            details: e.to_string(),
        }
    }
}

impl From<CseErrorReply> for HttpResponse {
    fn from(e: CseErrorReply) -> Self {
        debug!("CSE Error: {:?}", e);
        match e.code {
            400 => Self::BadRequest().json(e),
            401 => Self::Unauthorized().json(e),
            403 => Self::Forbidden().json(e),
            404 => Self::NotFound().json(e),
            405 => Self::MethodNotAllowed().json(e),
            422 => Self::UnprocessableEntity().json(e),
            _ => Self::InternalServerError().json(e),
        }
    }
}

/// Get the status for Google CSE
#[get("/status")]
pub async fn get_status(
    req: HttpRequest,
    kms: Data<Arc<KMSServer>>,
) -> KResult<Json<operations::StatusResponse>> {
    info!("GET /google_cse/status {}", kms.get_user(req)?);
    Ok(Json(operations::get_status()))
}

#[derive(Deserialize, Debug)]
pub struct DigestRequest {
    pub authorization: String,
    pub reason: String,
    pub wrapped_key: String,
}
#[post("/digest")]
pub async fn digest(
    _req_http: HttpRequest,
    _request: Json<DigestRequest>,
    _cse_config: Data<Option<GoogleCseConfig>>,
    _kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/digest: not implemented yet");
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
pub struct PrivilegedPrivateKeyDecryptRequest {
    pub authentication: String,
    pub algorithm: String,
    pub encrypted_data_encryption_key: String,
    pub rsa_oaep_label: String,
    pub reason: String,
    pub spki_hash: String,
    pub spki_hash_algorithm: String,
    pub wrapped_private_key: String,
}
#[post("/privilegedprivatekeydecrypt")]
pub async fn privilegedprivatekeydecrypt(
    _req_http: HttpRequest,
    _request: Json<PrivilegedPrivateKeyDecryptRequest>,
    _cse_config: Data<Option<GoogleCseConfig>>,
    _kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/privilegedprivatekeydecrypt: not implemented yet");
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
pub struct PrivilegedUnwrapRequest {
    pub authentication: String,
    pub reason: String,
    pub resource_name: String,
    pub wrapped_key: String,
}
#[post("/privilegedunwrap")]
pub async fn privilegedunwrap(
    _req_http: HttpRequest,
    _request: Json<PrivilegedUnwrapRequest>,
    _cse_config: Data<Option<GoogleCseConfig>>,
    _kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/privilegedunwrap: not implemented yet");
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
pub struct PrivilegedWrapRequest {
    pub authentication: String,
    pub key: String,
    pub perimeter_id: String,
    pub reason: String,
    pub resource_name: String,
}
#[post("/privilegedwrap")]
pub async fn privilegedwrap(
    _req_http: HttpRequest,
    _request: Json<PrivilegedWrapRequest>,
    _cse_config: Data<Option<GoogleCseConfig>>,
    _kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/privilegedwrap: not implemented yet");
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
pub struct RewrapRequest {
    pub authorization: String,
    pub original_kacls_url: String,
    pub reason: String,
    pub wrapped_key: String,
}
#[post("/rewrap")]
pub async fn rewrap(
    _req_http: HttpRequest,
    _request: Json<RewrapRequest>,
    _cse_config: Data<Option<GoogleCseConfig>>,
    _kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/rewrap: not implemented yet");
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
pub struct WrapPrivateKeyRequest {
    pub authentication: String,
    pub perimeter_id: String,
    pub private_key: String,
}
#[post("/wrapprivatekey")]
pub async fn wrapprivatekey(
    _req_http: HttpRequest,
    _request: Json<WrapPrivateKeyRequest>,
    _cse_config: Data<Option<GoogleCseConfig>>,
    _kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/wrapprivatekey: not implemented yet");
    HttpResponse::Ok().finish()
}

/// Returns encrypted Data Encryption Key (DEK) and associated data.
///
/// See [doc](https://developers.google.com/workspace/cse/reference/wrap) and
/// for more details, see [Encrypt & decrypt data](https://developers.google.com/workspace/cse/guides/encrypt-and-decrypt-data)
#[post("/wrap")]
pub async fn wrap(
    req_http: HttpRequest,
    wrap_request: Json<operations::WrapRequest>,
    cse_config: Data<Option<GoogleCseConfig>>,
    kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/wrap");

    let wrap_request = wrap_request.into_inner();
    trace!("wrap_request: {:?}", wrap_request);
    let kms = kms.into_inner();
    let cse_config = cse_config.into_inner();

    match operations::wrap(req_http, wrap_request, &cse_config, &kms)
        .await
        .map(Json)
    {
        Ok(wrap_response) => HttpResponse::Ok().json(wrap_response),
        Err(e) => CseErrorReply::from(e).into(),
    }
}

/// Decrypt the Data Encryption Key (DEK) and associated data.
///
/// See [doc](https://developers.google.com/workspace/cse/reference/wrap) and
/// for more details, see [Encrypt & decrypt data](https://developers.google.com/workspace/cse/guides/encrypt-and-decrypt-data)
#[post("/unwrap")]
pub async fn unwrap(
    req_http: HttpRequest,
    unwrap_request: Json<operations::UnwrapRequest>,
    cse_config: Data<Option<GoogleCseConfig>>,
    kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/unwrap");

    // unwrap all calls parameters
    let unwrap_request = unwrap_request.into_inner();
    trace!("unwrap_request: {:?}", unwrap_request);
    let kms = kms.into_inner();
    let cse_config = cse_config.into_inner();

    match operations::unwrap(req_http, unwrap_request, &cse_config, &kms)
        .await
        .map(Json)
    {
        Ok(wrap_response) => HttpResponse::Ok().json(wrap_response),
        Err(e) => CseErrorReply::from(e).into(),
    }
}

/// Unwraps a wrapped private key and then signs the digest provided by the client.
///
/// See [doc](https://developers.google.com/workspace/cse/reference/private-key-sign)
#[post("/privatekeysign")]
pub async fn private_key_sign(
    req_http: HttpRequest,
    request: Json<operations::PrivateKeySignRequest>,
    cse_config: Data<Option<GoogleCseConfig>>,
    kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/privatekeysign");

    // unwrap all calls parameters
    let request = request.into_inner();
    trace!("request: {request:?}");
    let kms = kms.into_inner();
    let cse_config = cse_config.into_inner();

    match operations::private_key_sign(req_http, request, &cse_config, &kms)
        .await
        .map(Json)
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => CseErrorReply::from(e).into(),
    }
}

/// Unwraps a wrapped private key and then decrypts the content encryption key that is encrypted to the public key.
///
/// See [doc](https://developers.google.com/workspace/cse/reference/private-key-decrypt)
#[post("/privatekeydecrypt")]
pub async fn private_key_decrypt(
    req_http: HttpRequest,
    request: Json<operations::PrivateKeyDecryptRequest>,
    cse_config: Data<Option<GoogleCseConfig>>,
    kms: Data<Arc<KMSServer>>,
) -> HttpResponse {
    info!("POST /google_cse/privatekeydecrypt");

    // unwrap all calls parameters
    let request = request.into_inner();
    trace!("request: {request:?}");
    let kms = kms.into_inner();
    let cse_config = cse_config.into_inner();

    match operations::private_key_decrypt(req_http, request, &cse_config, &kms)
        .await
        .map(Json)
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => CseErrorReply::from(e).into(),
    }
}

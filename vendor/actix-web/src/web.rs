//! Essentials helper functions and types for application registration.
use actix_http::http::Method;
use actix_router::IntoPattern;
use std::future::Future;

pub use actix_http::Response as HttpResponse;
pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use futures_channel::oneshot::Canceled;

use crate::error::BlockingError;
use crate::extract::FromRequest;
use crate::handler::Factory;
use crate::resource::Resource;
use crate::responder::Responder;
use crate::route::Route;
use crate::scope::Scope;
use crate::service::WebService;

pub use crate::config::ServiceConfig;
pub use crate::data::Data;
pub use crate::request::HttpRequest;
pub use crate::request_data::ReqData;
pub use crate::types::*;

/// Create resource for a specific path.
///
/// Resources may have variable path segments. For example, a
/// resource with the path `/a/{name}/c` would match all incoming
/// requests with paths such as `/a/b/c`, `/a/1/c`, or `/a/etc/c`.
///
/// A variable segment is specified in the form `{identifier}`,
/// where the identifier can be used later in a request handler to
/// access the matched value for that segment. This is done by
/// looking up the identifier in the `Params` object returned by
/// `HttpRequest.match_info()` method.
///
/// By default, each segment matches the regular expression `[^{}/]+`.
///
/// You can also specify a custom regex in the form `{identifier:regex}`:
///
/// For instance, to route `GET`-requests on any route matching
/// `/users/{userid}/{friend}` and store `userid` and `friend` in
/// the exposed `Params` object:
///
/// ```rust
/// # extern crate actix_web;
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/users/{userid}/{friend}")
///         .route(web::get().to(|| HttpResponse::Ok()))
///         .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
/// );
/// ```
pub fn resource<T: IntoPattern>(path: T) -> Resource {
    Resource::new(path)
}

/// Configure scope for common root path.
///
/// Scopes collect multiple paths under a common path prefix.
/// Scope path can contain variable path segments as resources.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::scope("/{project_id}")
///         .service(web::resource("/path1").to(|| HttpResponse::Ok()))
///         .service(web::resource("/path2").to(|| HttpResponse::Ok()))
///         .service(web::resource("/path3").to(|| HttpResponse::MethodNotAllowed()))
/// );
/// ```
///
/// In the above example, three routes get added:
///  * /{project_id}/path1
///  * /{project_id}/path2
///  * /{project_id}/path3
///
pub fn scope(path: &str) -> Scope {
    Scope::new(path)
}

/// Create *route* without configuration.
pub fn route() -> Route {
    Route::new()
}

/// Create *route* with `GET` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///        .route(web::get().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `GET` route gets added:
///  * /{project_id}
///
pub fn get() -> Route {
    method(Method::GET)
}

/// Create *route* with `POST` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::post().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `POST` route gets added:
///  * /{project_id}
///
pub fn post() -> Route {
    method(Method::POST)
}

/// Create *route* with `PUT` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::put().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `PUT` route gets added:
///  * /{project_id}
///
pub fn put() -> Route {
    method(Method::PUT)
}

/// Create *route* with `PATCH` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::patch().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `PATCH` route gets added:
///  * /{project_id}
///
pub fn patch() -> Route {
    method(Method::PATCH)
}

/// Create *route* with `DELETE` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::delete().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `DELETE` route gets added:
///  * /{project_id}
///
pub fn delete() -> Route {
    method(Method::DELETE)
}

/// Create *route* with `HEAD` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::head().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `HEAD` route gets added:
///  * /{project_id}
///
pub fn head() -> Route {
    method(Method::HEAD)
}

/// Create *route* with `TRACE` method guard.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::trace().to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `HEAD` route gets added:
///  * /{project_id}
///
pub fn trace() -> Route {
    method(Method::TRACE)
}

/// Create *route* and add method guard.
///
/// ```rust
/// use actix_web::{web, http, App, HttpResponse};
///
/// let app = App::new().service(
///     web::resource("/{project_id}")
///         .route(web::method(http::Method::GET).to(|| HttpResponse::Ok()))
/// );
/// ```
///
/// In the above example, one `GET` route gets added:
///  * /{project_id}
///
pub fn method(method: Method) -> Route {
    Route::new().method(method)
}

/// Create a new route and add handler.
///
/// ```rust
/// use actix_web::{web, App, HttpResponse, Responder};
///
/// async fn index() -> impl Responder {
///    HttpResponse::Ok()
/// }
///
/// App::new().service(
///     web::resource("/").route(
///         web::to(index))
/// );
/// ```
pub fn to<F, I, R, U>(handler: F) -> Route
where
    F: Factory<I, R, U>,
    I: FromRequest + 'static,
    R: Future<Output = U> + 'static,
    U: Responder + 'static,
{
    Route::new().to(handler)
}

/// Create raw service for a specific path.
///
/// ```rust
/// use actix_web::{dev, web, guard, App, Error, HttpResponse};
///
/// async fn my_service(req: dev::ServiceRequest) -> Result<dev::ServiceResponse, Error> {
///     Ok(req.into_response(HttpResponse::Ok().finish()))
/// }
///
/// let app = App::new().service(
///     web::service("/users/*")
///         .guard(guard::Header("content-type", "text/plain"))
///         .finish(my_service)
/// );
/// ```
pub fn service<T: IntoPattern>(path: T) -> WebService {
    WebService::new(path)
}

/// Execute blocking function on a thread pool, returns future that resolves
/// to result of the function execution.
pub async fn block<F, I, E>(f: F) -> Result<I, BlockingError<E>>
where
    F: FnOnce() -> Result<I, E> + Send + 'static,
    I: Send + 'static,
    E: Send + std::fmt::Debug + 'static,
{
    actix_threadpool::run(f).await
}

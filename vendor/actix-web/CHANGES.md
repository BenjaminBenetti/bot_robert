# Changes

## Unreleased - 2020-xx-xx


## 3.3.2 - 2020-12-01
### Fixed
* Removed an occasional `unwrap` on `None` panic in `NormalizePathNormalization`. [#1762]
* Fix `match_pattern()` returning `None` for scope with empty path resource. [#1798]
* Increase minimum `socket2` version. [#1803]

[#1762]: https://github.com/actix/actix-web/pull/1762
[#1798]: https://github.com/actix/actix-web/pull/1798
[#1803]: https://github.com/actix/actix-web/pull/1803


## 3.3.1 - 2020-11-29
* Ensure `actix-http` dependency uses same `serde_urlencoded`.


## 3.3.0 - 2020-11-25
### Added
* Add `Either<A, B>` extractor helper. [#1788]

### Changed
* Upgrade `serde_urlencoded` to `0.7`. [#1773]

[#1773]: https://github.com/actix/actix-web/pull/1773
[#1788]: https://github.com/actix/actix-web/pull/1788


## 3.2.0 - 2020-10-30
### Added
* Implement `exclude_regex` for Logger middleware. [#1723]
* Add request-local data extractor `web::ReqData`. [#1748]
* Add ability to register closure for request middleware logging. [#1749]
* Add `app_data` to `ServiceConfig`. [#1757]
* Expose `on_connect` for access to the connection stream before request is handled. [#1754]

### Changed
* Updated actix-web-codegen dependency for access to new `#[route(...)]` multi-method macro.
* Print non-configured `Data<T>` type when attempting extraction. [#1743]
* Re-export bytes::Buf{Mut} in web module. [#1750]
* Upgrade `pin-project` to `1.0`.

[#1723]: https://github.com/actix/actix-web/pull/1723
[#1743]: https://github.com/actix/actix-web/pull/1743
[#1748]: https://github.com/actix/actix-web/pull/1748
[#1750]: https://github.com/actix/actix-web/pull/1750
[#1754]: https://github.com/actix/actix-web/pull/1754
[#1749]: https://github.com/actix/actix-web/pull/1749


## 3.1.0 - 2020-09-29
### Changed
* Add `TrailingSlash::MergeOnly` behaviour to `NormalizePath`, which allows `NormalizePath`
  to retain any trailing slashes. [#1695]
* Remove bound `std::marker::Sized` from `web::Data` to support storing `Arc<dyn Trait>`
  via `web::Data::from` [#1710]

### Fixed
* `ResourceMap` debug printing is no longer infinitely recursive. [#1708]

[#1695]: https://github.com/actix/actix-web/pull/1695
[#1708]: https://github.com/actix/actix-web/pull/1708
[#1710]: https://github.com/actix/actix-web/pull/1710


## 3.0.2 - 2020-09-15
### Fixed
* `NormalizePath` when used with `TrailingSlash::Trim` no longer trims the root path "/". [#1678]

[#1678]: https://github.com/actix/actix-web/pull/1678


## 3.0.1 - 2020-09-13
### Changed
* `middleware::normalize::TrailingSlash` enum is now accessible. [#1673]

[#1673]: https://github.com/actix/actix-web/pull/1673


## 3.0.0 - 2020-09-11
* No significant changes from `3.0.0-beta.4`.


## 3.0.0-beta.4 - 2020-09-09
### Added
* `middleware::NormalizePath` now has configurable behaviour for either always having a trailing
  slash, or as the new addition, always trimming trailing slashes. [#1639]

### Changed
* Update actix-codec and actix-utils dependencies. [#1634]
* `FormConfig` and `JsonConfig` configurations are now also considered when set
  using `App::data`. [#1641]
* `HttpServer::maxconn` is renamed to the more expressive `HttpServer::max_connections`. [#1655]
* `HttpServer::maxconnrate` is renamed to the more expressive
  `HttpServer::max_connection_rate`. [#1655]

[#1639]: https://github.com/actix/actix-web/pull/1639
[#1641]: https://github.com/actix/actix-web/pull/1641
[#1634]: https://github.com/actix/actix-web/pull/1634
[#1655]: https://github.com/actix/actix-web/pull/1655

## 3.0.0-beta.3 - 2020-08-17
### Changed
* Update `rustls` to 0.18


## 3.0.0-beta.2 - 2020-08-17
### Changed
* `PayloadConfig` is now also considered in `Bytes` and `String` extractors when set
  using `App::data`. [#1610]
* `web::Path` now has a public representation: `web::Path(pub T)` that enables
  destructuring. [#1594]
* `ServiceRequest::app_data` allows retrieval of non-Data data without splitting into parts to
  access `HttpRequest` which already allows this. [#1618]
* Re-export all error types from `awc`. [#1621]
* MSRV is now 1.42.0.

### Fixed
* Memory leak of app data in pooled requests. [#1609]

[#1594]: https://github.com/actix/actix-web/pull/1594
[#1609]: https://github.com/actix/actix-web/pull/1609
[#1610]: https://github.com/actix/actix-web/pull/1610
[#1618]: https://github.com/actix/actix-web/pull/1618
[#1621]: https://github.com/actix/actix-web/pull/1621


## 3.0.0-beta.1 - 2020-07-13
### Added
* Re-export `actix_rt::main` as `actix_web::main`.
* `HttpRequest::match_pattern` and `ServiceRequest::match_pattern` for extracting the matched
  resource pattern.
* `HttpRequest::match_name` and `ServiceRequest::match_name` for extracting matched resource name.

### Changed
* Fix actix_http::h1::dispatcher so it returns when HW_BUFFER_SIZE is reached. Should reduce peak memory consumption during large uploads. [#1550]
* Migrate cookie handling to `cookie` crate. Actix-web no longer requires `ring` dependency.
* MSRV is now 1.41.1

### Fixed
* `NormalizePath` improved consistency when path needs slashes added _and_ removed.


## 3.0.0-alpha.3 - 2020-05-21
### Added
* Add option to create `Data<T>` from `Arc<T>` [#1509]

### Changed
* Resources and Scopes can now access non-overridden data types set on App (or containing scopes) when setting their own data. [#1486]
* Fix audit issue logging by default peer address [#1485]
* Bump minimum supported Rust version to 1.40
* Replace deprecated `net2` crate with `socket2`

[#1485]: https://github.com/actix/actix-web/pull/1485
[#1509]: https://github.com/actix/actix-web/pull/1509

## [3.0.0-alpha.2] - 2020-05-08

### Changed

* `{Resource,Scope}::default_service(f)` handlers now support app data extraction. [#1452]
* Implement `std::error::Error` for our custom errors [#1422]
* NormalizePath middleware now appends trailing / so that routes of form /example/ respond to /example requests. [#1433]
* Remove the `failure` feature and support.

[#1422]: https://github.com/actix/actix-web/pull/1422
[#1433]: https://github.com/actix/actix-web/pull/1433
[#1452]: https://github.com/actix/actix-web/pull/1452
[#1486]: https://github.com/actix/actix-web/pull/1486


## [3.0.0-alpha.1] - 2020-03-11

### Added

* Add helper function for creating routes with `TRACE` method guard `web::trace()`
* Add convenience functions `test::read_body_json()` and `test::TestRequest::send_request()` for testing.

### Changed

* Use `sha-1` crate instead of unmaintained `sha1` crate
* Skip empty chunks when returning response from a `Stream` [#1308]
* Update the `time` dependency to 0.2.7
* Update `actix-tls` dependency to 2.0.0-alpha.1
* Update `rustls` dependency to 0.17

[#1308]: https://github.com/actix/actix-web/pull/1308

## [2.0.0] - 2019-12-25

### Changed

* Rename `HttpServer::start()` to `HttpServer::run()`

* Allow to gracefully stop test server via `TestServer::stop()`

* Allow to specify multi-patterns for resources

## [2.0.0-rc] - 2019-12-20

### Changed

* Move `BodyEncoding` to `dev` module #1220

* Allow to set `peer_addr` for TestRequest #1074

* Make web::Data deref to Arc<T> #1214

* Rename `App::register_data()` to `App::app_data()`

* `HttpRequest::app_data<T>()` returns `Option<&T>` instead of `Option<&Data<T>>`

### Fixed

* Fix `AppConfig::secure()` is always false. #1202


## [2.0.0-alpha.6] - 2019-12-15

### Fixed

* Fixed compilation with default features off

## [2.0.0-alpha.5] - 2019-12-13

### Added

* Add test server, `test::start()` and `test::start_with()`

## [2.0.0-alpha.4] - 2019-12-08

### Deleted

* Delete HttpServer::run(), it is not useful with async/await

## [2.0.0-alpha.3] - 2019-12-07

### Changed

* Migrate to tokio 0.2


## [2.0.0-alpha.1] - 2019-11-22

### Changed

* Migrated to `std::future`

* Remove implementation of `Responder` for `()`. (#1167)


## [1.0.9] - 2019-11-14

### Added

* Add `Payload::into_inner` method and make stored `def::Payload` public. (#1110)

### Changed

* Support `Host` guards when the `Host` header is unset (e.g. HTTP/2 requests) (#1129)


## [1.0.8] - 2019-09-25

### Added

* Add `Scope::register_data` and `Resource::register_data` methods, parallel to
  `App::register_data`.

* Add `middleware::Condition` that conditionally enables another middleware

* Allow to re-construct `ServiceRequest` from `HttpRequest` and `Payload`

* Add `HttpServer::listen_uds` for ability to listen on UDS FD rather than path,
  which is useful for example with systemd.

### Changed

* Make UrlEncodedError::Overflow more informative

* Use actix-testing for testing utils


## [1.0.7] - 2019-08-29

### Fixed

* Request Extensions leak #1062


## [1.0.6] - 2019-08-28

### Added

* Re-implement Host predicate (#989)

* Form implements Responder, returning a `application/x-www-form-urlencoded` response

* Add `into_inner` to `Data`

* Add `test::TestRequest::set_form()` convenience method to automatically serialize data and set
  the header in test requests.

### Changed

* `Query` payload made `pub`. Allows user to pattern-match the payload.

* Enable `rust-tls` feature for client #1045

* Update serde_urlencoded to 0.6.1

* Update url to 2.1


## [1.0.5] - 2019-07-18

### Added

* Unix domain sockets (HttpServer::bind_uds) #92

* Actix now logs errors resulting in "internal server error" responses always, with the `error`
  logging level

### Fixed

* Restored logging of errors through the `Logger` middleware


## [1.0.4] - 2019-07-17

### Added

* Add `Responder` impl for `(T, StatusCode) where T: Responder`

* Allow to access app's resource map via
  `ServiceRequest::resource_map()` and `HttpRequest::resource_map()` methods.

### Changed

* Upgrade `rand` dependency version to 0.7


## [1.0.3] - 2019-06-28

### Added

* Support asynchronous data factories #850

### Changed

*  Use `encoding_rs` crate instead of unmaintained `encoding` crate


## [1.0.2] - 2019-06-17

### Changed

* Move cors middleware to `actix-cors` crate.

* Move identity middleware to `actix-identity` crate.


## [1.0.1] - 2019-06-17

### Added

* Add support for PathConfig #903

* Add `middleware::identity::RequestIdentity` trait to `get_identity` from `HttpMessage`.

### Changed

* Move cors middleware to `actix-cors` crate.

* Move identity middleware to `actix-identity` crate.

* Disable default feature `secure-cookies`.

* Allow to test an app that uses async actors #897

* Re-apply patch from #637 #894

### Fixed

* HttpRequest::url_for is broken with nested scopes #915


## [1.0.0] - 2019-06-05

### Added

* Add `Scope::configure()` method.

* Add `ServiceRequest::set_payload()` method.

* Add `test::TestRequest::set_json()` convenience method to automatically
  serialize data and set header in test requests.

* Add macros for head, options, trace, connect and patch http methods

### Changed

* Drop an unnecessary `Option<_>` indirection around `ServerBuilder` from `HttpServer`. #863

### Fixed

* Fix Logger request time format, and use rfc3339. #867

* Clear http requests pool on app service drop #860


## [1.0.0-rc] - 2019-05-18

### Add

* Add `Query<T>::from_query()` to extract parameters from a query string. #846
* `QueryConfig`, similar to `JsonConfig` for customizing error handling of query extractors.

### Changed

* `JsonConfig` is now `Send + Sync`, this implies that `error_handler` must be `Send + Sync` too.

### Fixed

* Codegen with parameters in the path only resolves the first registered endpoint #841


## [1.0.0-beta.4] - 2019-05-12

### Add

* Allow to set/override app data on scope level

### Changed

* `App::configure` take an `FnOnce` instead of `Fn`
* Upgrade actix-net crates


## [1.0.0-beta.3] - 2019-05-04

### Added

* Add helper function for executing futures `test::block_fn()`

### Changed

* Extractor configuration could be registered with `App::data()`
  or with `Resource::data()` #775

* Route data is unified with app data, `Route::data()` moved to resource
  level to `Resource::data()`

* CORS handling without headers #702

* Allow to construct `Data` instances to avoid double `Arc` for `Send + Sync` types.

### Fixed

* Fix `NormalizePath` middleware impl #806

### Deleted

* `App::data_factory()` is deleted.


## [1.0.0-beta.2] - 2019-04-24

### Added

* Add raw services support via `web::service()`

* Add helper functions for reading response body `test::read_body()`

* Add support for `remainder match` (i.e "/path/{tail}*")

* Extend `Responder` trait, allow to override status code and headers.

* Store visit and login timestamp in the identity cookie #502

### Changed

* `.to_async()` handler can return `Responder` type #792

### Fixed

* Fix async web::Data factory handling


## [1.0.0-beta.1] - 2019-04-20

### Added

* Add helper functions for reading test response body,
 `test::read_response()` and test::read_response_json()`

* Add `.peer_addr()` #744

* Add `NormalizePath` middleware

### Changed

* Rename `RouterConfig` to `ServiceConfig`

* Rename `test::call_success` to `test::call_service`

* Removed `ServiceRequest::from_parts()` as it is unsafe to create from parts.

* `CookieIdentityPolicy::max_age()` accepts value in seconds

### Fixed

* Fixed `TestRequest::app_data()`


## [1.0.0-alpha.6] - 2019-04-14

### Changed

* Allow to use any service as default service.

* Remove generic type for request payload, always use default.

* Removed `Decompress` middleware. Bytes, String, Json, Form extractors
  automatically decompress payload.

* Make extractor config type explicit. Add `FromRequest::Config` associated type.


## [1.0.0-alpha.5] - 2019-04-12

### Added

* Added async io `TestBuffer` for testing.

### Deleted

* Removed native-tls support


## [1.0.0-alpha.4] - 2019-04-08

### Added

* `App::configure()` allow to offload app configuration to different methods

* Added `URLPath` option for logger

* Added `ServiceRequest::app_data()`, returns `Data<T>`

* Added `ServiceFromRequest::app_data()`, returns `Data<T>`

### Changed

* `FromRequest` trait refactoring

* Move multipart support to actix-multipart crate

### Fixed

* Fix body propagation in Response::from_error. #760


## [1.0.0-alpha.3] - 2019-04-02

### Changed

* Renamed `TestRequest::to_service()` to `TestRequest::to_srv_request()`

* Renamed `TestRequest::to_response()` to `TestRequest::to_srv_response()`

* Removed `Deref` impls

### Removed

* Removed unused `actix_web::web::md()`


## [1.0.0-alpha.2] - 2019-03-29

### Added

* rustls support

### Changed

* use forked cookie

* multipart::Field renamed to MultipartField

## [1.0.0-alpha.1] - 2019-03-28

### Changed

* Complete architecture re-design.

* Return 405 response if no matching route found within resource #538

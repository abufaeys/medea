//! REST [Control API] mock server implementation.
//!
//! [Control API]: https://tinyurl.com/yxsqplq7

pub mod endpoint;
pub mod member;
pub mod room;

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer,
};
use clap::ArgMatches;
use futures::Future;
use medea_control_api_proto::grpc::control_api::{
    CreateResponse as CreateResponseProto, Element as ElementProto,
    Error as ErrorProto, GetResponse as GetResponseProto,
    Response as ResponseProto, Room_Element as RoomElementProto,
};
use serde::{Deserialize, Serialize};

use crate::{client::ControlClient, prelude::*};

use self::{
    endpoint::{WebRtcPlayEndpoint, WebRtcPublishEndpoint},
    member::Member,
    room::Room,
};

/// Context of [`actix_web`] server.
pub struct Context {
    /// Client for Medea's Control API.
    client: ControlClient,
}

/// Run REST Control API server mock.
pub fn run(args: &ArgMatches) {
    let medea_addr: String = args.value_of("medea_addr").unwrap().to_string();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new())
            .data(Context {
                client: ControlClient::new(&medea_addr),
            })
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to_async(batch_get))
                    .route(web::delete().to_async(batch_delete)),
            )
            .service(web::resource("hb").route(web::get().to_async(heartbeat)))
            .service(
                web::resource("/{room_id}")
                    .route(web::delete().to_async(room::delete))
                    .route(web::post().to_async(room::create))
                    .route(web::get().to_async(room::get)),
            )
            .service(
                web::resource("/{room_id}/{member_id}")
                    .route(web::delete().to_async(member::delete))
                    .route(web::post().to_async(member::create))
                    .route(web::get().to_async(member::get)),
            )
            .service(
                web::resource("/{room_id}/{member_id}/{endpoint_id}")
                    .route(web::delete().to_async(endpoint::delete))
                    .route(web::post().to_async(endpoint::create))
                    .route(web::get().to_async(endpoint::get)),
            )
    })
    .bind(args.value_of("addr").unwrap())
    .unwrap()
    .start();
}

/// `GET /hb`
///
/// Checks connection with Medea's gRPC Control API.
#[allow(clippy::needless_pass_by_value)]
pub fn heartbeat(
    state: Data<Context>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    state
        .client
        .get_single("")
        .map_err(|_| ())
        .map(|_| HttpResponse::Ok().body("Ok".to_string()))
}

/// Batch ID's request. Used for batch delete and get.
#[derive(Deserialize, Debug)]
pub struct BatchIdsRequest {
    /// Elements ids.
    ids: Vec<String>,
}

/// `GET /`
///
/// Batch get elements. With this method you can get getheterogeneous set of
/// elements.
#[allow(clippy::needless_pass_by_value)]
pub fn batch_get(
    state: Data<Context>,
    data: Json<BatchIdsRequest>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    state
        .client
        .get_batch(data.ids.clone())
        .map_err(|e| error!("{:?}", e))
        .map(|r| GetResponse::from(r).into())
}

/// `DELETE /`
///
/// Batch delete elements. With this method you can delete getheterogeneous set
/// of elements.
#[allow(clippy::needless_pass_by_value)]
pub fn batch_delete(
    state: Data<Context>,
    data: Json<BatchIdsRequest>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    state
        .client
        .delete_batch(data.0.ids)
        .map_err(|e| error!("{:?}", e))
        .map(|r| Response::from(r).into())
}

/// Error object. Returns when some error happened on Control API's side.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Medea's Control API error code.
    pub code: u32,

    /// Text of error.
    pub text: String,

    /// Element's ID with which error happened.
    pub element: String,
}

impl Into<ErrorResponse> for ErrorProto {
    fn into(mut self) -> ErrorResponse {
        ErrorResponse {
            code: self.get_code(),
            text: self.take_text(),
            element: self.take_element(),
        }
    }
}

/// Response which return sids.
///
/// Used for create methods.
#[derive(Debug, Serialize)]
pub struct CreateResponse {
    /// URIs with which Jason can connect `Member`s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<HashMap<String, String>>,

    /// Error if something happened on Control API's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// Response which can return only error (if any).
///
/// Used for delete methods.
#[derive(Debug, Serialize)]
pub struct Response {
    /// Error if something happened on Control API's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// Macro which implements [`Into`] [`HttpResponse`] for all
/// `control-api-mock` responses.
///
/// Implementation will check existence of `error` and if it exists then
/// [`HttpResponse`] will be `BadRequest` with this struct as response in
/// otherwise `Ok` with this struct as response.
macro_rules! impl_into_http_response {
    ($resp:tt) => {
        impl Into<HttpResponse> for $resp {
            fn into(self) -> HttpResponse {
                if self.error.is_some() {
                    HttpResponse::BadRequest().json(self)
                } else {
                    HttpResponse::Ok().json(self)
                }
            }
        }
    };
}

impl_into_http_response!(CreateResponse);
impl_into_http_response!(Response);
impl_into_http_response!(GetResponse);
impl_into_http_response!(SingleGetResponse);

impl From<ResponseProto> for Response {
    fn from(mut resp: ResponseProto) -> Self {
        if resp.has_error() {
            Self {
                error: Some(resp.take_error().into()),
            }
        } else {
            Self { error: None }
        }
    }
}

impl From<CreateResponseProto> for CreateResponse {
    fn from(mut resp: CreateResponseProto) -> Self {
        if resp.has_error() {
            Self {
                sids: None,
                error: Some(resp.take_error().into()),
            }
        } else {
            Self {
                sids: Some(resp.take_sid()),
                error: None,
            }
        }
    }
}

/// Union of all elements which exists in medea.
#[derive(Serialize, Debug)]
#[serde(tag = "kind")]
pub enum Element {
    Member(Member),
    WebRtcPublishEndpoint(WebRtcPublishEndpoint),
    WebRtcPlayEndpoint(WebRtcPlayEndpoint),
    Room(Room),
}

impl From<ElementProto> for Element {
    fn from(mut proto: ElementProto) -> Self {
        if proto.has_room() {
            Self::Room(proto.take_room().into())
        } else if proto.has_member() {
            Self::Member(proto.take_member().into())
        } else if proto.has_webrtc_pub() {
            Self::WebRtcPublishEndpoint(proto.take_webrtc_pub().into())
        } else if proto.has_webrtc_play() {
            Self::WebRtcPlayEndpoint(proto.take_webrtc_play().into())
        } else {
            unimplemented!()
        }
    }
}

impl From<RoomElementProto> for Element {
    fn from(mut proto: RoomElementProto) -> Self {
        if proto.has_member() {
            Self::Member(proto.take_member().into())
        } else {
            unimplemented!()
        }
    }
}

impl Into<RoomElementProto> for Element {
    fn into(self) -> RoomElementProto {
        let mut proto = RoomElementProto::new();
        match self {
            Self::Member(m) => proto.set_member(m.into()),
            _ => unimplemented!(),
        }
        proto
    }
}

/// Response on request for batch get `Element`s.
#[derive(Serialize, Debug)]
pub struct GetResponse {
    /// Requested elements.
    ///
    /// Key - element's ID
    ///
    /// Value - requested element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<HashMap<String, Element>>,

    /// Error if something happened on Control API's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

impl From<GetResponseProto> for GetResponse {
    fn from(mut proto: GetResponseProto) -> Self {
        if proto.has_error() {
            return Self {
                elements: None,
                error: Some(proto.take_error().into()),
            };
        }

        let mut elements = HashMap::new();
        for (id, element) in proto.take_elements() {
            elements.insert(id, element.into());
        }

        Self {
            elements: Some(elements),
            error: None,
        }
    }
}

/// Response on request for get single `Element`s.
#[derive(Serialize, Debug)]
pub struct SingleGetResponse {
    /// Requested element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Element>,

    /// Error if something happened on Control API's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

impl From<GetResponseProto> for SingleGetResponse {
    fn from(mut proto: GetResponseProto) -> Self {
        if proto.has_error() {
            Self {
                element: None,
                error: Some(proto.take_error().into()),
            }
        } else {
            Self {
                error: None,
                element: proto
                    .take_elements()
                    .into_iter()
                    .map(|(_, e)| e.into())
                    .next(),
            }
        }
    }
}

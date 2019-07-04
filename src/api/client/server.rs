//! HTTP server for handling WebSocket connections of Client API.

use actix_web::{
    middleware,
    web::{Data, Path, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use futures::{
    future::{self, Either},
    Future,
};
use serde::Deserialize;

use crate::{
    api::{
        client::{
            rpc_connection::{AuthorizationError, Authorize},
            session::WsSession,
        },
        control::model::{MemberId, RoomId},
    },
    conf::{Conf, Rpc},
    log::prelude::*,
    signalling::room_repo::RoomsRepository,
};

/// Parameters of new WebSocket connection creation HTTP request.
#[derive(Debug, Deserialize)]
struct RequestParams {
    /// ID of [`Room`] that WebSocket connection connects to.
    room_id: RoomId,
    /// ID of [`Member`] that establishes WebSocket connection.
    member_id: MemberId,
    /// Credential of [`Member`] to authorize WebSocket connection with.
    credentials: String,
}

/// Handles all HTTP requests, performs WebSocket handshake (upgrade) and starts
/// new [`WsSession`] for WebSocket connection.
fn ws_index(
    request: HttpRequest,
    info: Path<RequestParams>,
    state: Data<Context>,
    payload: Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    debug!("Request params: {:?}", info);

    match state.rooms.get(&info.room_id) {
        Some(room) => Either::A(
            room.send(Authorize {
                member_id: info.member_id.clone(),
                credentials: info.credentials.clone(),
            })
            .from_err()
            .and_then(move |res| match res {
                Ok(_) => ws::start(
                    WsSession::new(
                        info.member_id.clone(),
                        room,
                        state.config.idle_timeout,
                    ),
                    &request,
                    payload,
                ),
                Err(AuthorizationError::MemberNotExists) => {
                    Ok(HttpResponse::NotFound().into())
                }
                Err(AuthorizationError::InvalidCredentials) => {
                    Ok(HttpResponse::Forbidden().into())
                }
            }),
        ),
        None => Either::B(future::ok(HttpResponse::NotFound().into())),
    }
}

/// Context for [`App`] which holds all the necessary dependencies.
pub struct Context {
    /// Repository of all currently existing [`Room`]s in application.
    pub rooms: RoomsRepository,

    /// Settings of application.
    pub config: Rpc,
}

/// Starts HTTP server for handling WebSocket connections of Client API.
pub fn run(rooms: RoomsRepository, config: Conf) {
    let server_addr = config.server.bind_addr();
    HttpServer::new(move || {
        App::new()
            .data(Context {
                rooms: rooms.clone(),
                config: config.rpc.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(
                actix_web::web::resource(
                    "/ws/{room_id}/{member_id}/{credentials}",
                )
                .route(actix_web::web::get().to_async(ws_index)),
            )
    })
    .bind(server_addr)
    .unwrap()
    .start();

    info!("Started HTTP server on 0.0.0.0:8080");
}

#[cfg(test)]
mod test {
    use std::{ops::Add, thread, time::Duration};

    use actix::{Actor as _, Arbiter};
    use actix_http::{ws::Message, HttpService};
    use actix_http_test::{TestServer, TestServerRuntime};
    use actix_web::App;
    use futures::{future::IntoFuture as _, sink::Sink as _, Stream as _};

    use crate::{
        api::control,
        conf::{Conf, Server, Turn},
        signalling::Room,
        turn::new_turn_auth_service_mock,
    };

    use super::*;
    use crate::api::control::{
        model::room::RoomSpec, serde::room::ParsedSerdeRoomSpec,
    };

    /// Creates [`RoomsRepository`] for tests filled with a single [`Room`].
    fn room(conf: Rpc) -> RoomsRepository {
        let room_spec = control::serde::load_from_yaml_file(
            "tests/specs/pub_sub_video_call.yml",
        )
        .unwrap();

        let room_id = room_spec.id.clone();
        let client_room = Room::start_in_arbiter(&Arbiter::new(), move |_| {
            let room_spec = ParsedSerdeRoomSpec::new(&room_spec).unwrap();
            let room_spec = Box::new(&room_spec as &RoomSpec);
            let client_room = Room::new(
                &room_spec,
                conf.reconnect_timeout,
                new_turn_auth_service_mock(),
            )
            .unwrap();
            client_room
        });
        let room_hash_map = hashmap! {
            room_id => client_room,
        };

        RoomsRepository::new(room_hash_map)
    }

    /// Creates test WebSocket server of Client API which can handle requests.
    fn ws_server(conf: Conf) -> TestServerRuntime {
        TestServer::new(move || {
            HttpService::new(
                App::new()
                    .data(Context {
                        rooms: room(conf.rpc.clone()),
                        config: conf.rpc.clone(),
                    })
                    .service(
                        actix_web::web::resource(
                            "/ws/{room_id}/{member_id}/{credentials}",
                        )
                        .route(actix_web::web::get().to_async(ws_index)),
                    ),
            )
        })
    }

    #[test]
    fn ping_pong_and_disconnects_on_idle() {
        let conf = Conf {
            rpc: Rpc {
                idle_timeout: Duration::new(2, 0),
                reconnect_timeout: Default::default(),
            },
            turn: Turn::default(),
            server: Server::default(),
        };

        let mut server = ws_server(conf.clone());
        let socket =
            server.ws_at("/ws/pub-sub-video-call/caller/test").unwrap();

        server
            .block_on(
                socket
                    .send(Message::Text(r#"{"ping": 33}"#.into()))
                    .into_future()
                    .map_err(|e| panic!("{:?}", e))
                    .and_then(|socket| {
                        socket
                            .into_future()
                            .map_err(|(e, _)| panic!("{:?}", e))
                            .and_then(|(item, read)| {
                                assert_eq!(
                                    Some(ws::Frame::Text(Some(
                                        r#"{"pong":33}"#.into()
                                    ))),
                                    item
                                );

                                thread::sleep(
                                    conf.rpc
                                        .idle_timeout
                                        .add(Duration::from_secs(1)),
                                );

                                read.into_future()
                                    .map_err(|(e, _)| panic!("{:?}", e))
                                    .map(|(item, _)| {
                                        assert_eq!(
                                            Some(ws::Frame::Close(Some(
                                                ws::CloseCode::Normal.into()
                                            ))),
                                            item
                                        );
                                    })
                            })
                    }),
            )
            .unwrap();
    }
}

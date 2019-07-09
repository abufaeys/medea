pub mod webrtc_play_endpoint;
pub mod webrtc_publish_endpoint;

use std::convert::TryFrom;

use crate::api::control::grpc::protos::control::Member_Element as MemberElementProto;

use super::{Element, TryFromElementError, TryFromProtobufError};

use self::{
    webrtc_play_endpoint::WebRtcPlayEndpoint,
    webrtc_publish_endpoint::WebRtcPublishEndpoint,
};

/// [`Endpoint`] represents a media element that one or more media data streams
/// flow through.
#[derive(Debug)]
pub enum Endpoint {
    WebRtcPublish(WebRtcPublishEndpoint),
    WebRtcPlay(WebRtcPlayEndpoint),
}

impl Into<Element> for Endpoint {
    fn into(self) -> Element {
        match self {
            Endpoint::WebRtcPublish(e) => {
                Element::WebRtcPublishEndpoint { spec: e }
            }
            Endpoint::WebRtcPlay(e) => Element::WebRtcPlayEndpoint { spec: e },
        }
    }
}

impl TryFrom<&MemberElementProto> for Endpoint {
    type Error = TryFromProtobufError;

    fn try_from(value: &MemberElementProto) -> Result<Self, Self::Error> {
        if value.has_webrtc_play() {
            let play = WebRtcPlayEndpoint::try_from(value.get_webrtc_play())?;
            return Ok(Endpoint::WebRtcPlay(play));
        } else if value.has_webrtc_pub() {
            let publish =
                WebRtcPublishEndpoint::try_from(value.get_webrtc_pub())?;
            return Ok(Endpoint::WebRtcPublish(publish));
        } else {
            // TODO
            unimplemented!()
        }
    }
}

impl TryFrom<&Element> for Endpoint {
    type Error = TryFromElementError;

    fn try_from(from: &Element) -> Result<Self, Self::Error> {
        match from {
            Element::WebRtcPlayEndpoint { spec } => {
                Ok(Endpoint::WebRtcPlay(spec.clone()))
            }
            Element::WebRtcPublishEndpoint { spec } => {
                Ok(Endpoint::WebRtcPublish(spec.clone()))
            }
            _ => Err(TryFromElementError::NotEndpoint),
        }
    }
}
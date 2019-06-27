use crate::api::client::rpc_connection::RpcConnection;

pub use crate::api::control::MemberId;

#[derive(Debug)]
pub struct Member {
    id: MemberId,
    credentials: String,
    connection: Option<Box<dyn RpcConnection>>
}

impl Member {
    pub fn connection(&self) -> Option<&Box<dyn RpcConnection>> {
        self.connection.as_ref()
    }

    pub fn take_connection(&mut self) -> Option<Box<dyn RpcConnection>> {
        self.connection.take()
    }

    pub fn id(&self) -> &MemberId {
        &self.id
    }
}

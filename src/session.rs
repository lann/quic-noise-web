use std::any::Any;

use quinn_proto::{
    crypto::{ExportKeyingMaterialError, HeaderKey, KeyPair, Keys, PacketKey},
    transport_parameters::TransportParameters,
    ConnectionId, Side, TransportError,
};

pub struct Session;

impl quinn_proto::crypto::Session for Session {
    fn initial_keys(&self, dst_cid: &ConnectionId, side: Side) -> Keys {
        todo!()
    }

    fn handshake_data(&self) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn peer_identity(&self) -> Option<Box<dyn Any>> {
        todo!()
    }

    fn early_crypto(&self) -> Option<(Box<dyn HeaderKey>, Box<dyn PacketKey>)> {
        todo!()
    }

    fn early_data_accepted(&self) -> Option<bool> {
        todo!()
    }

    fn is_handshaking(&self) -> bool {
        todo!()
    }

    fn read_handshake(&mut self, buf: &[u8]) -> Result<bool, TransportError> {
        todo!()
    }

    fn transport_parameters(&self) -> Result<Option<TransportParameters>, TransportError> {
        todo!()
    }

    fn write_handshake(&mut self, buf: &mut Vec<u8>) -> Option<Keys> {
        todo!()
    }

    fn next_1rtt_keys(&mut self) -> Option<KeyPair<Box<dyn PacketKey>>> {
        todo!()
    }

    fn is_valid_retry(&self, orig_dst_cid: &ConnectionId, header: &[u8], payload: &[u8]) -> bool {
        todo!()
    }

    fn export_keying_material(
        &self,
        output: &mut [u8],
        label: &[u8],
        context: &[u8],
    ) -> Result<(), ExportKeyingMaterialError> {
        todo!()
    }
}

use protocols::protocol_types::ProtocolTypes;
use protocols::protocol::Protocol;
use protocols::version1_protocol::Version1Protocol;
use protocols::version2_protocol::Version2Protocol;

pub struct ProtocolFactory {}

impl ProtocolFactory {
    pub fn get_protocol(proto_type: ProtocolTypes) -> Box<Protocol> {
        match proto_type {
            ProtocolTypes::Version1 => Box::new(Version1Protocol{}),
            ProtocolTypes::Version2 => Box::new(Version2Protocol{}),
        }
    }
}
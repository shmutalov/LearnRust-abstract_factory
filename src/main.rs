mod protocols;

use protocols::protocol_types::ProtocolTypes;
use protocols::protocol_factory::ProtocolFactory;

fn main() {
    let proto = ProtocolFactory::get_protocol(ProtocolTypes::Version1);
    println!("{}", proto.get_name());

    let proto = ProtocolFactory::get_protocol(ProtocolTypes::Version2);
    println!("{}", proto.get_name());
}

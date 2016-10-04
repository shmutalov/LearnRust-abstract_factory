use protocols::protocol::Protocol;

pub struct Version1Protocol {}

impl Protocol for Version1Protocol {
    fn get_name(&self) -> String {
        "Version1".to_string()
    }
}
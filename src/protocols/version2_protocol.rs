use protocols::protocol::Protocol;

pub struct Version2Protocol {}

impl Protocol for Version2Protocol {
    fn get_name(&self) -> String {
        "Version2".to_string()
    }
}
use ext_php_rs::convert::FromZval;
use ext_php_rs::flags::DataType;
use ext_php_rs::types::Zval;
use std::collections::HashMap;
use tiktoken_rs::FunctionCall;

pub type Message = HashMap<String, MessageValue>;

pub enum MessageValue {
    String(String),
    HashMap(HashMap<String, String>),
}

impl<'a> FromZval<'a> for MessageValue {
    const TYPE: DataType = DataType::Mixed;

    fn from_zval(zval: &'a Zval) -> Option<Self> {
        if zval.is_string() {
            return Some(MessageValue::String(zval.string().unwrap()));
        }
        if zval.is_array() {
            return Some(MessageValue::HashMap(
                zval.array()
                    .unwrap()
                    .iter()
                    .filter(|(_, key, val)| key.is_some() && val.is_string())
                    .map(|(_, key, val)| (key.unwrap(), val.string().unwrap()))
                    .collect(),
            ));
        }
        None
    }
}

impl MessageValue {
    pub fn string(&self) -> Option<String> {
        match self {
            MessageValue::String(string) => Some(string.to_string()),
            _ => None,
        }
    }

    pub fn hashmap(&self) -> Option<&HashMap<String, String>> {
        match self {
            MessageValue::HashMap(hashmap) => Some(hashmap),
            _ => None,
        }
    }
}

pub fn get_message_string(message: &Message, key: &str) -> Option<String> {
    Some(message.get(key)?.string()?)
}

pub fn get_message_function_call(message: &Message) -> Option<FunctionCall> {
    let function_call = message.get("function_call")?.hashmap()?;

    Some(FunctionCall {
        name: function_call.get("name")?.to_string(),
        arguments: function_call.get("arguments")?.to_string(),
    })
}

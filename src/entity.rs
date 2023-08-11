use ext_php_rs::convert::FromZval;
use ext_php_rs::flags::DataType;
use ext_php_rs::types::Zval;
use std::collections::HashMap;
use tiktoken_rs::tokenizer::Tokenizer;
use tiktoken_rs::FunctionCall;

pub type Message = HashMap<String, MessageValue>;

pub trait MessageAccessor {
    fn string(&self, key: &str) -> Option<String>;
    fn function_call(&self) -> Option<FunctionCall>;
}

impl MessageAccessor for Message {
    fn string(&self, key: &str) -> Option<String> {
        Some(self.get(key)?.string()?)
    }

    fn function_call(&self) -> Option<FunctionCall> {
        let function_call = self.get("function_call")?.hashmap()?;

        Some(FunctionCall {
            name: function_call.get("name")?.to_string(),
            arguments: function_call.get("arguments")?.to_string(),
        })
    }
}

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
        if let MessageValue::String(string) = self {
            Some(string.to_string())
        } else {
            None
        }
    }

    pub fn hashmap(&self) -> Option<&HashMap<String, String>> {
        if let MessageValue::HashMap(hashmap) = self {
            Some(hashmap)
        } else {
            None
        }
    }
}

pub fn get_tokenizer(tokenizer: &str) -> Option<Tokenizer> {
    match tokenizer {
        "gpt2" => Some(Tokenizer::Gpt2),
        "r50k_base" => Some(Tokenizer::R50kBase),
        "p50k_base" => Some(Tokenizer::P50kBase),
        "p50k_edit" => Some(Tokenizer::P50kEdit),
        "cl100k_base" => Some(Tokenizer::Cl100kBase),
        _ => None,
    }
}

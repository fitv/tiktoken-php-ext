#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::{php_function, php_module, ModuleBuilder};
use std::collections::HashMap;
use tiktoken_rs::{
    get_bpe_from_model, num_tokens_from_messages as _num_tokens_from_messages,
    ChatCompletionRequestMessage,
};

/// Get the number of tokens in the text.
#[php_function(name = "TikToken\\num_tokens")]
pub fn num_tokens(model: String, text: String) -> usize {
    get_bpe_from_model(model.as_str())
        .unwrap()
        .encode_with_special_tokens(text.as_str())
        .len()
}

/// Get the number of tokens in the message.
#[php_function(name = "TikToken\\num_tokens_from_messages")]
pub fn num_tokens_from_messages(model: String, messages: Vec<HashMap<String, String>>) -> usize {
    let messages: Vec<ChatCompletionRequestMessage> = messages
        .iter()
        .filter(|message| message.contains_key("role") && message.contains_key("content"))
        .map(|message| ChatCompletionRequestMessage {
            role: message.get("role").unwrap().to_string(),
            content: Some(message.get("content").unwrap().to_string()),
            name: None,
            function_call: None,
        })
        .collect();
    _num_tokens_from_messages(model.as_str(), &messages).unwrap()
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

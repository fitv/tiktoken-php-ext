#![cfg_attr(windows, feature(abi_vectorcall))]

mod entity;

use entity::{get_message_function_call, get_message_string, Message};
use ext_php_rs::prelude::{php_function, php_module, ModuleBuilder};
use tiktoken_rs::model::get_context_size;
use tiktoken_rs::{get_bpe_from_model, ChatCompletionRequestMessage};

/// Get the maximum token number of a specified model.
#[php_function(name = "TikToken\\model_max_tokens")]
pub fn model_max_tokens(model: String) -> usize {
    get_context_size(model.as_str())
}

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
pub fn num_tokens_from_messages(model: String, messages: Vec<Message>) -> usize {
    let messages: Vec<_> = messages
        .iter()
        .filter(|message| get_message_string(message, "role").is_some())
        .map(|message| ChatCompletionRequestMessage {
            role: get_message_string(message, "role").unwrap(),
            content: get_message_string(message, "content"),
            name: get_message_string(message, "name"),
            function_call: get_message_function_call(message),
        })
        .collect();
    tiktoken_rs::num_tokens_from_messages(model.as_str(), &messages).unwrap()
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

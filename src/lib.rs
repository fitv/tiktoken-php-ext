#![cfg_attr(windows, feature(abi_vectorcall))]

mod entity;

use entity::{get_tokenizer, Message, MessageAccessor};
use ext_php_rs::prelude::{php_function, php_module, ModuleBuilder, PhpResult};
use tiktoken_rs::model::get_context_size;
use tiktoken_rs::{get_bpe_from_model, get_bpe_from_tokenizer, ChatCompletionRequestMessage};

/// Encode the text into a list of tokens.
#[php_function(name = "TikToken\\encode")]
pub fn encode(tokenizer: String, text: String) -> PhpResult<Vec<usize>> {
    let tokenizer =
        get_tokenizer(&tokenizer).ok_or(format!("Tokenizer {} not found", tokenizer))?;
    Ok(get_bpe_from_tokenizer(tokenizer)?.encode_with_special_tokens(text.as_str()))
}

/// Decode the tokens into a text.
#[php_function(name = "TikToken\\decode")]
pub fn decode(tokenizer: String, tokens: Vec<usize>) -> PhpResult<String> {
    let tokenizer =
        get_tokenizer(&tokenizer).ok_or(format!("Tokenizer {} not found", tokenizer))?;
    Ok(get_bpe_from_tokenizer(tokenizer)?.decode(tokens)?)
}

/// Encode the text into a list of tokens for a specified model.
#[php_function(name = "TikToken\\encode_for_model")]
pub fn encode_for_model(model: String, text: String) -> PhpResult<Vec<usize>> {
    Ok(get_bpe_from_model(model.as_str())?.encode_with_special_tokens(text.as_str()))
}

/// Decode the tokens into a text for a specified model.
#[php_function(name = "TikToken\\decode_for_model")]
pub fn decode_for_model(model: String, tokens: Vec<usize>) -> PhpResult<String> {
    Ok(get_bpe_from_model(model.as_str())?.decode(tokens)?)
}

/// Get the maximum token number of a specified model.
#[php_function(name = "TikToken\\model_max_tokens")]
pub fn model_max_tokens(model: String) -> usize {
    get_context_size(model.as_str())
}

/// Get the number of tokens in the text.
#[php_function(name = "TikToken\\num_tokens")]
pub fn num_tokens(model: String, text: String) -> PhpResult<usize> {
    Ok(get_bpe_from_model(model.as_str())?
        .encode_with_special_tokens(text.as_str())
        .len())
}

/// Get the number of tokens in the message.
#[php_function(name = "TikToken\\num_tokens_from_messages")]
pub fn num_tokens_from_messages(model: String, messages: Vec<Message>) -> PhpResult<usize> {
    let messages: Vec<_> = messages
        .iter()
        .filter(|message| message.string("role").is_some())
        .map(|message| ChatCompletionRequestMessage {
            role: message.string("role").unwrap(),
            content: message.string("content"),
            name: message.string("name"),
            function_call: message.function_call(),
        })
        .collect();
    Ok(tiktoken_rs::num_tokens_from_messages(
        model.as_str(),
        &messages,
    )?)
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}

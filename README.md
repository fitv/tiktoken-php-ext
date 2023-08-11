# tiktoken-php-ext

A [tiktoken](https://github.com/openai/tiktoken) extension for PHP.

## Requirements

- Rust >= 1.57
- PHP >= 8.0
- Clang >= 5.0

## Installation

Build the extension with cargo.
```bash
cargo build --release
```

Move the generated library to your php extension directory.
```
cp ./target/release/libtiktoken.so $(php-config --extension-dir)/tiktoken.so
```

Then add `extension=tiktoken.so` to your php.ini file.
```
echo "extension=tiktoken.so" >> $(php --ini | grep "Loaded Configuration" | sed -e "s|.*:\s*||")
```

Check if the extension is loaded.
```bash
php -m | grep tiktoken
```

## Usage

```php
<?php

use function TikToken\decode;
use function TikToken\decode_for_model;
use function TikToken\encode;
use function TikToken\encode_for_model;
use function TikToken\model_max_tokens;
use function TikToken\num_tokens;
use function TikToken\num_tokens_from_messages;

var_dump(decode('cl100k_base', encode('cl100k_base', 'Hello!')) === 'Hello!');

var_dump(decode_for_model('gpt-4', encode_for_model('gpt-4', 'Hello!')) === 'Hello!');

var_dump(model_max_tokens('gpt-4'));

var_dump(num_tokens('gpt-4', 'Hello!'));

var_dump(num_tokens_from_messages('gpt-4', [
    [
        'role' => 'system',
        'content' => 'You are a helpful assistant.',
    ],
    [
        'role' => 'user',
        'content' => 'What is your name?',
    ],
    [
        'role' => 'assistant',
        'content' => 'My name is ChatGPT.',
    ],
]));
```

Stub file for IDEs.
```php
<?php

namespace TikToken {
    /**
     * Encode the text into a list of tokens.
     */
    function encode(string $tokenizer, string $text): array {}

    /**
     * Decode the tokens into a text.
     */
    function decode(string $tokenizer, array $tokens): string {}

    /**
     * Encode the text into a list of tokens for a specified model.
     */
    function encode_for_model(string $model, string $text): array {}

    /**
     * Decode the tokens into a text for a specified model.
     */
    function decode_for_model(string $model, array $tokens): string {}

    /**
     * Get the maximum token number of a specified model.
     */
    function model_max_tokens(string $model): int {}

    /**
     * Get the number of tokens in the text.
     */
    function num_tokens(string $model, string $text): int {}

    /**
     * Get the number of tokens in the message.
     */
    function num_tokens_from_messages(string $model, array $messages): int {}
}
```

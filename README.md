# gpt-cli

gpt-cli is a Rust program that provides a chatbot using OpenAI's GPT-3.5-turbo model. The program takes input messages from the user and displays responses generated by the GPT-3.5-turbo model.

## Prerequisites

- Rust programming language installed
- OpenAI API key set in the OPENAI_API_KEY environment variable

## Usage

1. Clone or download this repository.
2. Navigate to the project directory in your terminal.
3. Run the program with the following command:

```
cargo run [OPTIONS] [MESSAGE]
```

### Options:

- -h, --help: Show the help message and exit.
- -t, --timeout: Set the request timeout in seconds (default: 30).
- -m, --message: Set the input message for the chatbot.
  `[MESSAGE]` will be used as the input message for the chatbot if no options are specified.

## Example

```
cargo run -t 60 -m "Hello, gpt-cli!"
```

This command sets the timeout to 60 seconds and sends the message "Hello, gpt-cli!" to the chatbot.

## Notes

Keep your OpenAI API key secret. Be careful not to accidentally expose it.
Using GPT-3.5-turbo may incur costs associated with your API key. Be mindful of your usage.

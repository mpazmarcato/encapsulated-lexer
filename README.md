# Lexical Analyzer (Encapsulated) - Rust

A simple Lexical Analyzer written in Rust, designed to tokenize arithmetic expressions with numbers and operators.
This version is encapsulated, meaning the tokenizer logic is implemented using the Analisador struct with private fields, exposing public methods to interact with tokens safely.
---

## Project Structure
```bash
src/
├── main.rs       # Entry point and example usage
├── analisador.rs # Tokenizer function (`Analisador`) implementation
└── iteradores.rs # Custom iterators for chars and char indices
```
---

## Features
- Tokenization of integers and arithmetic operators (+, -, *, /).
- Support for custom symbols (🦀, 🐧) as examples of extensibility.
- Handles whitespace automatically.
- Returns token positions and reports errors for invalid characters.
- Encapsulated design: state is private and accessed only through public methods.
- Custom iterators (Chars and CharIndices) for traversing strings.

---

## How It Works

- The main tokenizer is the Analisador struct.
- Use Analisador::new(input) to create a tokenizer instance.
- Call the method proximo() to get the next token:
```bash
match analisador.proximo() {
    Ok((position, token)) => { /* valid token */ }
    Err(None) => { /* end of input */ }
    Err(Some(pos)) => { /* error at position */ }
}
```

- position: the 1-based character position of the token.

- token: the string slice representing the token.

- Internal state (position and remaining string) is private, ensuring safe and consistent iteration.

- Iterators Chars and CharIndices allow safe traversal of characters and their byte indices in the string.

---

## Running the Analyzer

### 1. Clone the repository
```bash
git clone https://github.com/mpazmarcato/encapsulated-lexer.git
cd encapsulated-lexer
```

### 2. Run with Cargo
```bash
cargo run
```
```bash
Example Output
450 + 20
Token: 450, posição: 1
Token: +, posição: 5
Token: 20, posição: 7
Fim da análise

10 + 20a
Token: 10, posição: 1
Token: +, posição: 4
Erro na posição 7
```
---

## Requirements
- Rust 1.70+
- Cargo package manager

No external dependencies required

## License
This project is open-source and available under the MIT License.

## Author
María Marcato
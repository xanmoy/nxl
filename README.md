# AXE

> A Python-inspired programming language with explicit block syntax.

AXE is an experimental programming language designed around the simplicity of Python with a more explicit, brace-based syntax.

Instead of relying on indentation to define blocks, AXE uses `{}`.

```axe
fn greet(name) {
    return "Hello, " + name
}

let message = greet("Tanmoy")
print(message)
```

## 🚧 Project Status

AXE is currently under active development.

Current version: **0.1.0 — Lexer + Parser**

The current implementation can:

- Tokenize AXE source code
- Parse AXE source code into an Abstract Syntax Tree (AST)
- Handle variables
- Handle constants
- Handle functions
- Handle function calls
- Handle `if / else`
- Handle `return`
- Parse arithmetic expressions
- Parse comparison operators
- Parse logical operators
- Respect operator precedence
- Report lexer and parser errors

The runtime/interpreter is not implemented yet.

## Why AXE?

AXE is an experiment in building a programming language from the ground up.

The goal is to combine:

- Python's readability
- C-style block syntax
- Modern language features
- A simple and approachable syntax
- A clean compiler architecture

AXE is not intended to be Python with curly braces. Python is the inspiration, but AXE is intended to develop its own language design over time.

## Example

AXE:

```axe
let name = "Tanmoy"
let age = 23

fn greet(name) {
    return "Hello, " + name
}

if age >= 18 {
    print(greet(name))
} else {
    print("Too young")
}
```

## Architecture

Currently AXE uses a frontend pipeline:

```
AXE Source
    │
    ▼
  Lexer
    │
    ▼
  Tokens
    │
    ▼
  Parser
    │
    ▼
   AST
```

The planned runtime will extend this to:

```
AXE Source
    │
    ▼
  Lexer
    │
    ▼
  Parser
    │
    ▼
   AST
    │
    ▼
Interpreter
    │
    ▼
  Runtime
    │
    ▼
  Output
```

## Project Structure

```
axe/
├── axe/
│   ├── __init__.py
│   ├── ast.py
│   ├── errors.py
│   ├── lexer.py
│   ├── main.py
│   ├── parser.py
│   └── tokens.py
│
├── examples/
│   ├── hello.axe
│   └── expressions.axe
│
├── tests/
│   ├── test_lexer.py
│   └── test_parser.py
│
├── .gitignore
├── pyproject.toml
└── README.md
```

## Development

Clone the repository:

```bash
git clone <repository-url>
cd axe
```

Create a virtual environment:

```bash
python3 -m venv .venv
```

Activate it:

```bash
source .venv/bin/activate
```

Install AXE in editable mode:

```bash
python -m pip install -e .
```

Run an AXE program:

```bash
axe examples/hello.axe
```

## Running Tests

```bash
pytest
```

## Roadmap

### AXE 0.1 — Frontend

- [x] Token system
- [x] Lexer
- [x] AST
- [x] Parser
- [x] Error handling
- [x] CLI
- [x] Lexer tests
- [x] Parser tests

### AXE 0.2 — Interpreter

- [ ] Runtime
- [ ] Environment
- [ ] Variables
- [ ] Expressions
- [ ] Functions
- [ ] Function calls
- [ ] `if / else`
- [ ] Built-in `print`
- [ ] REPL

### AXE 0.3 — Data Structures

- [ ] Lists
- [ ] Maps
- [ ] Sets
- [ ] Indexing
- [ ] `for`
- [ ] `while`
- [ ] `break`
- [ ] `continue`

### Future

- [ ] Modules
- [ ] Standard library
- [ ] Error/result system
- [ ] Structs
- [ ] Classes
- [ ] Async / await
- [ ] Package manager
- [ ] Formatter
- [ ] Linter
- [ ] Bytecode VM
- [ ] Native compilation
- [ ] Python interoperability

## License

MIT License

Copyright (c) 2026 Tanmoy Ganguly
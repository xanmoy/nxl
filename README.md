# NXL

> A simple, expressive programming language built from scratch in Rust.

NXL is an experimental programming language designed to keep the simplicity of Python while introducing a more familiar C/Java/Rust-style syntax.

The most important syntax difference is:

```nxl
if age >= 18 {
    print("Adult")
}
```

NXL uses `{}` to define blocks instead of indentation.

The compiler/runtime is being built from scratch in Rust.

---

## Table of Contents

- [Project Status](#project-status)
- [Why NXL?](#why-nxl)
- [Example](#example)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Architecture Overview](#architecture-overview)
- [Complete Execution Pipeline](#complete-execution-pipeline)
  - [1. CLI](#srcmainrs)
  - [2. Source Code](#stage-1--source-code)
  - [3. Lexer](#stage-2--lexer)
  - [4. Tokens](#stage-3--tokens)
  - [5. Parser](#stage-4--parser)
  - [6. AST](#srcastrs)
  - [7. Runtime Values](#srcruntimers)
  - [8. Environment](#srcenvironmentrs)
  - [9. Interpreter](#srcinterpreterrs)
  - [10. Error Handling](#srcerrorrs)
- [How Everything Connects](#how-everything-connects)
- [Example: Complete Program Flow](#example-complete-program-flow)
- [Operator Precedence](#operator-precedence)
- [Scopes](#scopes)
- [Current Runtime](#current-runtime)
- [Current Limitations](#current-limitations)
- [Development Workflow](#development-workflow)
- [Building NXL](#building-nxl)
- [Testing](#testing)
- [Git Workflow](#git-workflow)
- [Roadmap](#roadmap)
- [Learning Guide](#learning-guide)
- [Contributing](#contributing)
- [License](#license)

---

## Project Status

NXL is currently under active development.

Current architecture:

```
Source Code
    │
    ▼
   CLI
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
    │
    ▼
Interpreter
    │
    ├── Environment
    │
    └── Runtime Values
    │
    ▼
  Output
```

**Implemented:**

- [x] Rust project
- [x] CLI file loading
- [x] Token system
- [x] Lexer
- [x] Keywords
- [x] Numbers
- [x] Strings
- [x] Booleans
- [x] `null`
- [x] Operators
- [x] Comments
- [x] `{}` blocks
- [x] Parser
- [x] AST
- [x] Operator precedence in parser
- [x] Variables
- [x] Runtime values
- [x] Environment
- [x] Variable lookup
- [x] Interpreter
- [x] Built-in `print()`

**Currently being developed:**

- [ ] Binary expression evaluation
- [ ] Comparison evaluation
- [ ] Boolean expressions
- [ ] if / else execution
- [ ] Assignment execution
- [ ] Functions
- [ ] Function calls
- [ ] `return`
- [ ] Arrays
- [ ] Loops
- [ ] Modules
- [ ] Standard library
- [ ] Tests
- [ ] Formatter
- [ ] Package manager

---

## Why NXL?

NXL started with a simple idea:

> What if Python had a simple syntax but used `{}` instead of indentation?

For example, Python:

```python
if age >= 18:
    print("Adult")
else:
    print("Minor")
```

NXL:

```nxl
if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}
```

The goal isn't to simply clone Python.

NXL is an experiment in understanding how programming languages work while designing a language with its own syntax, runtime, and eventually its own ecosystem.

---

## Example

A simple NXL program:

```nxl
let name = "Tanmoy"
let age = 23

print(name)
print(age)

if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}
```

Run it with:

```bash
cargo run -- examples/hello.nxl
```

The current interpreter supports basic values, variables, and `print()`.

---

## Getting Started

### Requirements

You need:

- Rust
- Cargo
- Git

Check your installation:

```bash
rustc --version
cargo --version
```

### Clone the repository

```bash
git clone <repository-url>
cd nxl
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run -- examples/hello.nxl
```

The `--` separates Cargo arguments from arguments passed to the NXL executable.

Conceptually:

```
cargo run -- examples/hello.nxl
           │
           └── passed to NXL
```

---

## Project Structure

Current project structure:

```
nxl/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── examples/
│   └── hello.nxl
│
└── src/
    │
    ├── main.rs
    ├── token.rs
    ├── lexer.rs
    ├── parser.rs
    ├── ast.rs
    ├── error.rs
    ├── runtime.rs
    ├── environment.rs
    └── interpreter.rs
```

Each file has a specific responsibility.

---

## Architecture Overview

NXL is divided into several stages.

```
NXL SOURCE
                      │
                      ▼
              ┌───────────────┐
              │     CLI       │
              │   main.rs     │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │     Lexer     │
              │   lexer.rs    │
              └───────┬───────┘
                      │
                      ▼
                   Tokens
                      │
                      ▼
              ┌───────────────┐
              │    Parser     │
              │  parser.rs    │
              └───────┬───────┘
                      │
                      ▼
                     AST
                      │
                      ▼
              ┌───────────────┐
              │  Interpreter  │
              │interpreter.rs │
              └───────┬───────┘
                      │
              ┌───────┴────────┐
              │                │
              ▼                ▼
       Environment          Runtime
      environment.rs       runtime.rs
              │                │
              └───────┬────────┘
                      │
                      ▼
                   OUTPUT
```

Each stage has one main responsibility.

---

## Complete Execution Pipeline

Suppose we have:

```nxl
let age = 23

if age >= 18 {
    print("Adult")
}
```

NXL processes this in several stages.

### Stage 1 — Source Code

The source starts as plain text:

```nxl
let age = 23

if age >= 18 {
    print("Adult")
}
```

At this point NXL knows nothing about the meaning of the program. It is just characters.

### Stage 2 — Lexer

The lexer reads characters and groups them into meaningful units.

```
let age = 23
```

becomes approximately:

```
LET
IDENTIFIER("age")
EQUAL
NUMBER(23)
NEWLINE
```

Then:

```
if age >= 18 {
```

becomes:

```
IF
IDENTIFIER("age")
GREATER_EQUAL
NUMBER(18)
LEFT_BRACE
```

### Stage 3 — Tokens

Tokens are represented by the types defined in `src/token.rs`.

A token contains information such as:

- TokenType
- Lexeme
- Literal
- Line
- Column

For example:

```rust
Token {
    token_type: Number,
    lexeme: "23",
    literal: Number(23.0),
    line: 1,
    column: 11
}
```

### Stage 4 — Parser

The parser takes the token stream and determines the structure of the program.

For example:

```
age >= 18
```

becomes:

```
Binary
├── Identifier("age")
├── GreaterEqual
└── Literal(18)
```

The parser also handles precedence. For:

```
a + b * 2
```

the parser produces:

```
      +
     / \
    a   *
       / \
      b   2
```

rather than:

```
      *
     / \
    +   2
   / \
  a   b
```

This means NXL understands: `a + (b * 2)`

---

### `src/main.rs`

`main.rs` is currently the NXL command-line entry point.

Its responsibilities are:

1. Read command-line arguments.
2. Validate the `.nxl` filename.
3. Read the file.
4. Create the lexer.
5. Tokenize the source.
6. Create the parser.
7. Parse the tokens.
8. Create the interpreter.
9. Execute the AST.

The current flow is:

```rust
let source = fs::read_to_string(filename)?;
```

Then:

```rust
let mut lexer = Lexer::new(&source);
```

Then:

```rust
let tokens = lexer.tokenize()?;
```

Then:

```rust
let mut parser = Parser::new(tokens);
```

Then:

```rust
let ast = parser.parse()?;
```

Finally:

```rust
let mut interpreter = Interpreter::new();
interpreter.interpret(&ast)?;
```

Therefore `main.rs` is the orchestrator. It doesn't implement language rules itself.

---

### `src/token.rs`

This file defines NXL's token system.

The lexer uses `TokenType` to classify source code.

Examples:

```
Identifier
Number
String

Let
Const
Fn
If
Else
Return

Plus
Minus
Star
Slash

Equal
EqualEqual
NotEqual

Greater
GreaterEqual
Less
LessEqual

LeftBrace
RightBrace
LeftParen
RightParen
```

**Literal**

Literal values represent values directly present in source code.

Examples:

```
123
3.14
"hello"
true
false
null
```

They are represented by the `Literal` enum.

**Token**

A `Token` contains:

- token type
- source text
- literal value
- line
- column

This allows later stages to know not only what something is, but where it came from.

---

### `src/lexer.rs`

The lexer converts raw source code into tokens.

This process is called **lexical analysis**.

Example:

```
let x = 10
```

Input:

```
'l' 'e' 't' ' ' 'x' ' ' '=' ' ' '1' '0'
```

Output:

```
LET
IDENTIFIER("x")
EQUAL
NUMBER(10)
```

**Lexer responsibilities**

The lexer understands:

*Keywords*

```
let const fn if else for while return true false null and or not
```

*Operators*

```
+ - * / % = == != > >= < <=
```

*Delimiters*

```
( ) { } [ ] , . :
```

*Strings*

```
"Hello"
'Tanmoy'
```

*Numbers*

```
123
42.5
```

*Comments*

```
// This is a comment
```

*Identifiers*

```
name
age
my_variable
```

---

### `src/parser.rs`

The parser converts tokens into an AST.

This process is called **syntactic analysis**.

The parser understands NXL's grammar. For example:

```
let x = 10
```

means:

```
VariableDeclaration
├── name: x
└── initializer: 10
```

**Parser precedence**

The parser implements precedence levels. Conceptually:

```
assignment
    │
    ▼
logical OR
    │
    ▼
logical AND
    │
    ▼
equality
    │
    ▼
comparison
    │
    ▼
term
    │
    ▼
factor
    │
    ▼
unary
    │
    ▼
call
    │
    ▼
primary
```

This allows `a + b * 2` to correctly become `a + (b * 2)`.

---

### `src/ast.rs`

AST stands for **Abstract Syntax Tree**.

It is the structured representation of NXL code.

The AST contains:

```
Program
Statement
Expression
LiteralValue
```

**Program**

A program contains statements:

```rust
Program {
    statements: Vec<Statement>
}
```

Example:

```nxl
let x = 10
let y = 20
```

becomes:

```
Program
├── VariableDeclaration
└── VariableDeclaration
```

**Statements**

NXL currently defines several statement types:

```
VariableDeclaration
Expression
Block
If
Function
Return
```

**Expressions**

NXL currently defines:

```
Literal
Identifier
Binary
Unary
Assignment
Call
```

**Example AST**

Source:

```nxl
let result = a + b * 2
```

AST:

```
VariableDeclaration
│
├── name: result
│
└── initializer
       │
       ▼
     Binary(+)
      /    \
     /      \
    a      Binary(*)
           /      \
          b        2
```

This tree is what the interpreter eventually walks.

---

### `src/runtime.rs`

The runtime defines values that exist while an NXL program executes.

Currently:

```rust
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
```

Therefore:

```nxl
let age = 23
```

can become:

```rust
Value::Number(23.0)
```

and:

```nxl
let name = "Tanmoy"
```

becomes:

```rust
Value::String("Tanmoy")
```

**Why separate runtime values from AST literals?**

The AST describes the program. The runtime describes values while the program runs.

For example, AST `Literal(Number(23))` becomes runtime `Value::Number(23.0)`.

This separation becomes increasingly important as the language grows.

Eventually runtime values may include:

```
Number
String
Boolean
Null
Array
Object
Function
NativeFunction
```

---

### `src/environment.rs`

The environment stores variables.

Conceptually:

```
Environment

name → "Tanmoy"
age  → 23
```

It uses a hash map internally.

**Define**

When the interpreter executes `let age = 23` it eventually performs the equivalent of:

```rust
define("age", Value::Number(23))
```

**Get**

When NXL evaluates `print(age)` the interpreter performs:

```rust
get("age")
```

and retrieves `Value::Number(23)`.

**Assignment**

The environment also has an `assign()` operation.

Eventually this will allow:

```nxl
let age = 23
age = 24
```

to update the existing variable.

**Parent environments**

Environments support parent scopes.

Conceptually:

```
Global Environment
│
├── x → 10
│
└── Block Environment
    │
    └── x → 20
```

This allows nested scopes. Example:

```nxl
let x = 10

if true {
    let x = 20
    print(x)
}

print(x)
```

Expected:

```
20
10
```

---

### `src/interpreter.rs`

The interpreter is the execution engine.

It takes the AST and evaluates it.

Current flow:

```
Program
   │
   ▼
Statement
   │
   ▼
Expression
   │
   ▼
Runtime Value
```

**Variable declaration**

For `let name = "Tanmoy"` the interpreter:

1. Evaluates `"Tanmoy"`.
2. Creates a runtime `Value`.
3. Stores it in the environment.

Conceptually:

```
" T a n m o y "
       ↓
Value::String
       ↓
Environment
       ↓
name → "Tanmoy"
```

**Variable lookup**

For `print(name)` the interpreter:

```
Identifier("name")
        ↓
Environment.get("name")
        ↓
Value::String("Tanmoy")
```

**Built-in `print()`**

NXL currently has one built-in function: `print()`.

Example:

```nxl
print("Hello")
```

The parser creates:

```
Call
├── callee: print
└── arguments:
    └── "Hello"
```

The interpreter recognizes `print` as a built-in function. It evaluates the argument and sends it to stdout.

Example:

```nxl
print(23)
```

Output:

```
23
```

---

### `src/error.rs`

This file defines NXL errors.

The current error structure contains:

- message
- line
- column

Example:

```
NXL ERROR: [line 5, column 10] Unexpected character '@'
```

Different stages can report different errors.

**Lexer errors**

Example:

```nxl
let x = @
```

The lexer can report an unexpected character.

**Parser errors**

Example:

```nxl
let =
```

The parser can report: `Expected variable name.`

**Runtime errors**

Example:

```nxl
print(unknown)
```

The interpreter can report: `Undefined variable 'unknown'.`

---

## How Everything Connects

This is the most important diagram in the project:

```
hello.nxl
                       │
                       │ source text
                       ▼
                ┌──────────────┐
                │   main.rs    │
                │     CLI      │
                └──────┬───────┘
                       │
                       ▼
                ┌──────────────┐
                │   lexer.rs   │
                │    Lexer     │
                └──────┬───────┘
                       │
                       │ Vec<Token>
                       ▼
                ┌──────────────┐
                │  parser.rs   │
                │    Parser    │
                └──────┬───────┘
                       │
                       │ Program
                       ▼
                ┌──────────────┐
                │    ast.rs    │
                │     AST      │
                └──────┬───────┘
                       │
                       ▼
             ┌────────────────────┐
             │  interpreter.rs    │
             │    Interpreter     │
             └─────────┬──────────┘
                       │
              ┌────────┴────────┐
              │                 │
              ▼                 ▼
       environment.rs     runtime.rs
       variable state      Value types
              │                 │
              └────────┬────────┘
                       │
                       ▼
                    stdout
```

---

## Example: Complete Program Flow

Consider:

```nxl
let name = "Tanmoy"
let age = 23

print(name)
```

**Step 1 — Lexer**

The lexer creates:

```
LET
IDENTIFIER("name")
EQUAL
STRING("Tanmoy")
NEWLINE

LET
IDENTIFIER("age")
EQUAL
NUMBER(23)
NEWLINE

IDENTIFIER("print")
LEFT_PAREN
IDENTIFIER("name")
RIGHT_PAREN
EOF
```

**Step 2 — Parser**

The parser creates:

```
Program
├── VariableDeclaration
│   ├── name
│   └── "Tanmoy"
│
├── VariableDeclaration
│   ├── age
│   └── 23
│
└── Expression
    └── Call
        ├── print
        └── name
```

**Step 3 — Interpreter**

First:

```
name = "Tanmoy"
```

Environment:

```
name → "Tanmoy"
```

Then:

```
age = 23
```

Environment:

```
name → "Tanmoy"
age  → 23
```

Then `print(name)`. The interpreter evaluates:

```
name
 ↓
Environment
 ↓
"Tanmoy"
```

Then `print()` produces:

```
Tanmoy
```

---

## Operator Precedence

The parser already understands operator precedence.

For:

```nxl
let result = a + b * 2
```

the AST is:

```
      +
     / \
    a   *
       / \
      b   2
```

Therefore `b * 2` is evaluated first.

Result: `a + (b * 2)`

This is handled by the parser, not the lexer.

The lexer only knows: `a + b * 2` as separate tokens. The parser determines the structure.

---

## Scopes

NXL uses environments to represent scopes.

Example:

```nxl
let x = 10

if true {
    let x = 20
    print(x)
}

print(x)
```

Conceptually:

```
Global
│
├── x = 10
│
└── If Block
    │
    └── x = 20
```

Inside the block: `x → 20`
Outside: `x → 10`

The environment's parent field allows nested scopes to find values from outer scopes.

---

## Current Runtime

At the current stage, NXL can execute:

```nxl
let name = "Tanmoy"
let age = 23

print(name)
print(age)
print("Hello, NXL")
print(true)
print(null)
```

Output:

```
Tanmoy
23
Hello, NXL
true
null
```

The interpreter currently understands:

- variable declarations
- literal values
- variable lookup
- expression statements
- blocks
- `print()`

The AST and parser already contain more functionality than the interpreter currently executes. This is intentional. The architecture allows language features to be implemented incrementally.

---

## Current Limitations

Some AST nodes exist but are not yet executed by the interpreter.

For example:

```
Binary
Unary
Assignment
If
Function
Return
```

This means code such as:

```nxl
let result = a + b
```

can already be parsed into an AST, but binary arithmetic is not yet evaluated by the runtime.

Similarly:

```nxl
if age >= 18 {
    print("Adult")
}
```

can be parsed, but the interpreter does not yet execute the conditional logic.

This distinction is important:

> Parser support ≠ Runtime support

A language feature normally needs to exist across multiple layers.

---

## Development Workflow

When implementing a new feature, consider all layers.

For example, adding `+` requires:

```
Lexer
  ↓
TokenType::Plus
  ↓
Parser
  ↓
Expression::Binary
  ↓
Interpreter
  ↓
Runtime Value
```

The lexer recognizes `+`. The parser understands where `+` belongs. The AST represents the operation. The interpreter evaluates it. The runtime provides the resulting value.

---

## Building NXL

Build the project:

```bash
cargo build
```

Build optimized:

```bash
cargo build --release
```

Check the project:

```bash
cargo check
```

Run:

```bash
cargo run -- examples/hello.nxl
```

### Running an NXL Program

Create `examples/hello.nxl`:

```nxl
let name = "Tanmoy"

print(name)
```

Run:

```bash
cargo run -- examples/hello.nxl
```

Eventually the project will provide an installed executable:

```bash
nxl examples/hello.nxl
```

---

## Testing

NXL will eventually have tests at multiple levels.

**Lexer tests**

Test: `source → tokens`

Example: `let x = 10` should produce:

```
LET
IDENTIFIER
EQUAL
NUMBER
```

**Parser tests**

Test: `tokens → AST`

For example, `a + b * 2` must produce `a + (b * 2)`.

**Interpreter tests**

Test: `AST → runtime result`

For example:

```nxl
let x = 10
print(x)
```

should produce:

```
10
```

**Integration tests**

Eventually:

```
.nxl file
   ↓
NXL CLI
   ↓
complete execution
   ↓
expected stdout
```

---

## Git Workflow

NXL is developed incrementally.

Each meaningful feature should have its own commit. Examples:

```bash
git add src/runtime.rs
git commit -m "feat: add NXL runtime value types"

git add src/environment.rs
git commit -m "feat: add NXL environment and scope handling"

git add src/interpreter.rs
git commit -m "feat: add initial NXL interpreter"

git commit -m "feat: add print builtin"
```

This makes the project history useful for understanding how the language evolved.

---

## Roadmap

### NXL 0.1 — Frontend

Completed:

- Lexer
- Parser
- AST
- CLI
- Error handling

### NXL 0.2 — Runtime

Current milestone:

- Runtime values
- Environment
- Interpreter
- Built-ins
- Arithmetic
- Comparisons
- Boolean logic
- Conditionals
- Variables
- Assignments
- Functions
- Return

Target:

```nxl
let name = "Tanmoy"
let age = 23

fn greet(name) {
    return "Hello, " + name
}

if age >= 18 {
    print(greet(name))
}
```

Output:

```
Hello, Tanmoy
```

### NXL 0.3 — Collections

Potential features:

```nxl
let numbers = [1, 2, 3, 4]

print(numbers[0])
```

Potential additions:

- Arrays
- Indexing
- Maps
- String methods
- Collection operations

### NXL 0.4 — Control Flow

Potential features:

- `for ...`
- `while ...`
- `break`
- `continue`

### NXL 0.5 — Functions & Modules

Potential features:

- Modules
- Imports
- Exports
- Standard library

Example:

```nxl
import math

print(math.sqrt(16))
```

### NXL 0.6 — Developer Experience

Potential tools:

```
nxl fmt
nxl check
nxl run
nxl build
```

And eventually:

- Formatter
- Linter
- Language Server
- Editor integrations
- Debugger

### NXL 0.7+ — Performance

The initial interpreter can eventually evolve into:

```
Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
Bytecode compiler
  ↓
NXL Virtual Machine
```

Potentially:

```
Source
  ↓
AST
  ↓
Bytecode
  ↓
VM
```

This can significantly improve execution performance.

---

## Learning Guide

NXL is intentionally structured so that someone learning how programming languages work can follow the pipeline stage by stage: source text, lexical analysis, parsing, AST construction, and tree-walking interpretation. Each source file maps to one stage of that pipeline, so reading the project in the order Source → Lexer → Parser → AST → Interpreter mirrors how the language actually executes a program.

---

## Contributing

Contributions are welcome. Please open an issue or pull request describing the change, and follow the incremental, single-feature-per-commit workflow described above.

---

## License

*MIT License.*
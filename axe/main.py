import sys

from .lexer import Lexer
from .parser import Parser
from .errors import AXEError


def main():
    if len(sys.argv) != 2:
        print("Usage: axe <file.axe>")
        sys.exit(1)

    filename = sys.argv[1]

    try:
        with open(filename, "r", encoding="utf-8") as file:
            source = file.read()

        # -----------------------------------------------------
        # Lexer
        # -----------------------------------------------------

        lexer = Lexer(source)
        tokens = lexer.tokenize()

        print("TOKENS")
        print("=" * 40)

        for token in tokens:
            print(token)

        # -----------------------------------------------------
        # Parser
        # -----------------------------------------------------

        parser = Parser(tokens)
        ast = parser.parse()

        print()
        print("AST")
        print("=" * 40)

        print(ast)

    except AXEError as error:
        print(f"AXE ERROR: {error}")
        sys.exit(1)


if __name__ == "__main__":
    main()
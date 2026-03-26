import type { Item, Size, ParseError } from "./types";

type Unit = Size["unit"];

export function parse(input: string): Item | ParseError {
  const parser = new Parser(input);
  try {
    const item = parser.parseItem();
    parser.skipWhitespace();
    if (parser.pos < parser.input.length) {
      return parser.error("unexpected input after layout");
    }
    return item;
  } catch (error) {
    if (isParseError(error)) return error;
    throw error;
  }
}

export function isParseError(value: unknown): value is ParseError {
  return (
    typeof value === "object" &&
    value !== null &&
    "message" in value &&
    "position" in value &&
    !("type" in value)
  );
}

class Parser {
  input: string;
  pos: number;

  constructor(input: string) {
    this.input = input;
    this.pos = 0;
  }

  parseItem(): Item {
    this.skipWhitespace();

    if (this.peek() === "(") {
      const item = this.parseGroup();
      const outerSize = this.tryParseSize();
      if (outerSize) {
        item.size = outerSize;
      }
      return item;
    }

    if (this.peek() === undefined) {
      throw this.error("expected a panel or group");
    }

    const node = this.parsePanel();
    const size = this.tryParseSize();
    return { ...node, ...(size && { size }) };
  }

  parseGroup(): Item {
    this.advance(); // consume '('
    this.skipWhitespace();

    if (this.peek() === ")") {
      throw this.error("empty group");
    }

    const first = this.parseItem();
    this.skipWhitespace();

    const next = this.peek();

    if (next === ")") {
      // Single-item group — collapse to inner item
      this.advance();
      return first;
    }

    if (next === "|" || next === "/") {
      const children = this.parseSplitTail(first, next);
      const type = next === "|" ? "h_split" : "v_split";
      return { type, children } as Item;
    }

    if (next === undefined) {
      throw this.error("unclosed group — expected ')'");
    }

    throw this.error(`expected '|', '/', or ')'`);
  }

  parseSplitTail(first: Item, operator: string): Item[] {
    const children = [first];
    const otherOperator = operator === "|" ? "/" : "|";

    while (this.peek() === operator) {
      this.advance();
      children.push(this.parseItem());
      this.skipWhitespace();
    }

    this.skipWhitespace();
    if (this.peek() === otherOperator) {
      throw this.error(
        `mixed operators — group uses '${operator}' but found '${otherOperator}'`
      );
    }

    if (this.peek() === ")") {
      this.advance();
      return children;
    }

    if (this.peek() === undefined) {
      throw this.error("unclosed group — expected ')'");
    }

    throw this.error("expected operator or ')'");
  }

  parsePanel(): Item {
    this.skipWhitespace();
    const character = this.peek();

    if (character === '"' || character === "'") {
      return this.parseQuoted();
    }

    return this.parseWord();
  }

  parseWord(): Item {
    const start = this.pos;

    while (this.pos < this.input.length) {
      const character = this.input[this.pos];
      if (/[\s|/()="']/.test(character)) break;
      this.pos++;
    }

    if (this.pos === start) {
      throw this.error("expected a panel name");
    }

    return { type: "panel", name: this.input.slice(start, this.pos) };
  }

  parseQuoted(): Item {
    const quote = this.advance();
    let name = "";

    while (true) {
      const character = this.peek();

      if (character === undefined) {
        throw this.error(`unclosed string — expected closing ${quote}`);
      }

      if (character === "\\") {
        this.advance();
        const escaped = this.peek();
        if (escaped === undefined) {
          throw this.error(`unclosed string — expected closing ${quote}`);
        }
        if (escaped === quote) {
          name += escaped;
          this.advance();
        } else {
          name += "\\" + escaped;
          this.advance();
        }
        continue;
      }

      if (character === quote) {
        this.advance();
        return { type: "panel", name };
      }

      name += character;
      this.advance();
    }
  }

  tryParseSize(): Size | null {
    this.skipWhitespace();
    if (this.peek() !== "=") return null;
    this.advance();
    this.skipWhitespace();

    const value = this.parseNumber();
    const unit = this.parseUnit();
    return { value, unit };
  }

  parseNumber(): number {
    const start = this.pos;

    while (this.pos < this.input.length && /\d/.test(this.input[this.pos])) {
      this.pos++;
    }

    if (this.pos < this.input.length && this.input[this.pos] === ".") {
      this.pos++;
      while (this.pos < this.input.length && /\d/.test(this.input[this.pos])) {
        this.pos++;
      }
    }

    if (this.pos === start) {
      throw this.error("expected a number");
    }

    return parseFloat(this.input.slice(start, this.pos));
  }

  parseUnit(): Unit {
    if (this.peek() === "%") {
      this.advance();
      return "%";
    }

    const start = this.pos;
    while (
      this.pos < this.input.length &&
      /[a-zA-Z]/.test(this.input[this.pos])
    ) {
      this.pos++;
    }

    const unitString = this.input.slice(start, this.pos);

    switch (unitString) {
      case "fr":
      case "col":
      case "row":
      case "px":
        return unitString;
      case "":
        throw this.error("size value requires a unit (fr, col, row, px, %)");
      default:
        throw this.error(
          `unknown unit '${unitString}' — expected fr, col, row, px, or %`
        );
    }
  }

  // -- Helpers --

  peek(): string | undefined {
    return this.input[this.pos];
  }

  advance(): string {
    const character = this.input[this.pos];
    this.pos++;
    return character;
  }

  skipWhitespace(): void {
    while (this.pos < this.input.length && /\s/.test(this.input[this.pos])) {
      this.pos++;
    }
  }

  error(message: string): ParseError {
    return { message, position: this.pos };
  }
}

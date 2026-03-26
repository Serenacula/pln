import { useState } from "preact/hooks";
import { parse, isParseError } from "../lib/parser";
import type { Item } from "../lib/types";
import LayoutRenderer from "./LayoutRenderer";

const EXAMPLES = [
  { label: "Horizontal split", value: "(left | right)" },
  { label: "Vertical split", value: "(top / bottom)" },
  { label: "Nested", value: "(editor | (terminal / files))" },
  { label: "Unequal split", value: "(left=2fr | right)" },
  { label: "Fixed sidebar", value: "(sidebar=80col | main)" },
  { label: "Three-way", value: "(nav=60col | content | panel=40col)" },
  { label: "IDE layout", value: "((files=20% | editor1 | editor2)=3fr / terminal)" },
  { label: "Quoted names", value: '("Left Panel"=2fr | "Right Panel")' },
];

export default function Playground() {
  const [input, setInput] = useState(EXAMPLES[6].value);
  const result = parse(input);
  const hasError = isParseError(result);

  return (
    <div class="playground-container">
      <div class="editor-pane">
        <div class="editor-header">
          <label for="pln-input">PLN expression</label>
          <select
            onChange={(event) => {
              const target = event.target as HTMLSelectElement;
              if (target.value) {
                setInput(target.value);
              }
            }}
          >
            <option value="">Examples...</option>
            {EXAMPLES.map((example) => (
              <option key={example.label} value={example.value}>
                {example.label}
              </option>
            ))}
          </select>
        </div>
        <textarea
          id="pln-input"
          value={input}
          onInput={(event) =>
            setInput((event.target as HTMLTextAreaElement).value)
          }
          spellcheck={false}
          autocomplete="off"
        />
        {hasError && <div class="error">{result.message}</div>}
      </div>

      <div class="preview-pane">
        <div class="preview-header">Preview</div>
        <div class="preview-area">
          {!hasError && <LayoutRenderer item={result as Item} />}
        </div>
      </div>
    </div>
  );
}

import type { Item, Size } from "../lib/types";

const PANEL_COLORS = [
  "var(--panel-1)",
  "var(--panel-2)",
  "var(--panel-3)",
  "var(--panel-4)",
  "var(--panel-5)",
  "var(--panel-6)",
  "var(--panel-7)",
  "var(--panel-8)",
];

let colorIndex = 0;

function resetColors() {
  colorIndex = 0;
}

function nextColor(): string {
  const color = PANEL_COLORS[colorIndex % PANEL_COLORS.length];
  colorIndex++;
  return color;
}

// Scale col/row to a 100x25 terminal grid relative to the container
// Using percentages so they adapt to any preview size
function sizeToFlex(size?: Size): string {
  if (!size) return "1";
  switch (size.unit) {
    case "fr":
      return `${size.value}`;
    case "%":
      return `0 0 ${size.value}%`;
    case "px":
      return `0 0 ${size.value}px`;
    case "col":
      return `0 0 ${size.value}%`; // 1 col = 1% of width (100-col terminal)
    case "row":
      return `0 0 ${size.value * 4}%`; // 1 row = 4% of height (25-row terminal)
  }
}

function sizeLabel(size?: Size): string {
  if (!size) return "";
  if (size.unit === "%") return `${size.value}%`;
  return `${size.value}${size.unit}`;
}

function RenderItem({ item }: { item: Item }) {
  if (item.type === "panel") {
    const background = nextColor();
    const label = sizeLabel(item.size);
    return (
      <div
        style={{
          flex: sizeToFlex(item.size),
          background,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          fontFamily: "var(--font-mono)",
          fontSize: "0.75rem",
          color: "var(--color-text-muted)",
          minWidth: 0,
          minHeight: 0,
          overflow: "hidden",
        }}
      >
        <span>{item.name}</span>
        {label && <span style={{ fontSize: "0.65rem", opacity: 0.7 }}>{label}</span>}
      </div>
    );
  }

  const direction = item.type === "h_split" ? "row" : "column";
  return (
    <div
      style={{
        flex: sizeToFlex(item.size),
        display: "flex",
        flexDirection: direction,
        minWidth: 0,
        minHeight: 0,
      }}
    >
      {item.children?.map((child, index) => (
        <RenderItem key={index} item={child} />
      ))}
    </div>
  );
}

export default function LayoutRenderer({ item }: { item: Item }) {
  resetColors();
  return (
    <div
      style={{
        width: "100%",
        height: "100%",
        display: "flex",
        borderRadius: "4px",
        overflow: "hidden",
      }}
    >
      <RenderItem item={item} />
    </div>
  );
}

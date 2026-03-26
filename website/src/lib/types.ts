export interface Item {
  type: "panel" | "h_split" | "v_split";
  name?: string;
  children?: Item[];
  size?: Size;
}

export interface Size {
  value: number;
  unit: "fr" | "col" | "row" | "px" | "%";
}

export interface ParseError {
  message: string;
  position: number;
}

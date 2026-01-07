// Utility to parse backticks in text and return parts for rendering
export interface TextPart {
  text: string;
  isCode: boolean;
}

export function parseBackticks(text: string): TextPart[] {
  const parts: TextPart[] = [];
  const regex = /`([^`]+)`/g;
  let lastIndex = 0;
  let match;

  while ((match = regex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push({ text: text.slice(lastIndex, match.index), isCode: false });
    }
    parts.push({ text: match[1], isCode: true });
    lastIndex = regex.lastIndex;
  }

  if (lastIndex < text.length) {
    parts.push({ text: text.slice(lastIndex), isCode: false });
  }

  // If no backticks found, return the original text
  if (parts.length === 0) {
    parts.push({ text, isCode: false });
  }

  return parts;
}

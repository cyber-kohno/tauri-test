// src/types/highlightjs.d.ts
declare module "highlight.js/lib/languages/typescript" {
  import { Language } from "highlight.js";
  const language: Language;
  export default language;
}

declare module "highlight.js/lib/languages/javascript" {
  import { Language } from "highlight.js";
  const language: Language;
  export default language;
}

/* 必要なら他の言語も同様に追加 */

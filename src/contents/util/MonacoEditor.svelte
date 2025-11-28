<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import loader from "@monaco-editor/loader";
  import type * as Monaco from "monaco-editor/esm/vs/editor/editor.api.js";

  let editorDiv: HTMLDivElement | null = null;
  let editor: Monaco.editor.IStandaloneCodeEditor;

  // 初期内容、言語、テーマなど
  export let value: string = "";
  export let language: string = "javascript";
  export let theme: string = "vs";
  export let onChange: (value: string) => void = () => {};

  export let declares: string[] = [];

  let monaco: typeof Monaco;

  onMount(async () => {
    if (!editorDiv) return;

    const monacoEditor = await import("monaco-editor");
    loader.config({ monaco: monacoEditor.default });
    monaco = await loader.init();

    // ① 型定義を投げ込む
    const typescript = monaco.languages.typescript as any;
    typescript.typescriptDefaults.addExtraLib(`${declares.join()}`, "global.d.ts");

    // Your monaco instance is ready, let's display some code!
    editor = monaco.editor.create(editorDiv, {
      value,
      language,
      theme,
      automaticLayout: true, // 例：ウィンドウリサイズでレイアウトを再計算
      // …任意のオプション
    });

    // 内容変更時にイベントを発火
    editor.onDidChangeModelContent(() => {
      const newVal = editor?.getValue() ?? "";
      onChange(newVal); // ← ここがコールバック呼び出し
    });
  });

  onDestroy(() => {
    editor?.dispose();
  });
</script>

<div bind:this={editorDiv} class="editor-wrapper"></div>

<style>
  .editor-wrapper {
    width: 100%;
    height: 100%;
  }
</style>

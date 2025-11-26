<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import type * as monaco from 'monaco-editor';

  let editorDiv: HTMLDivElement | null = null;
  let editor: monaco.editor.IStandaloneCodeEditor | null = null;

  // 初期内容、言語、テーマなど
  export let value: string = '';
  export let language: string = 'javascript';
  export let theme: string = 'vs-dark';
  export let options: monaco.editor.IStandaloneEditorConstructionOptions = {};

  // 変更イベントを外部に流す
  const dispatch = createEventDispatcher();

  onMount(async () => {
    if (!editorDiv) return;

    // monaco がグローバルに存在するので型だけ import しておく
    const monacoLib = await import('monaco-editor');

    editor = monacoLib.editor.create(editorDiv, {
      value,
      language,
      theme,
      automaticLayout: true,
      ...options
    });

    // 内容変更時にイベントを発火
    editor.onDidChangeModelContent(() => {
      const newVal = editor?.getValue() ?? '';
      dispatch('change', { value: newVal });
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

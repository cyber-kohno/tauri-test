<script lang="ts">
    import { onMount } from "svelte";
    import hljs from "highlight.js/lib/common";

    // 必要な言語だけインポート
    import javascript from "highlight.js/lib/languages/typescript";
    import typescript from "highlight.js/lib/languages/typescript";
    import python from "highlight.js/lib/languages/python";

    hljs.registerLanguage("javascript", javascript as any);
    hljs.registerLanguage("typescript", typescript as any);
    hljs.registerLanguage("python", python);

    export let code: string = "";
    export let language: string = "javascript";

    let preEl: HTMLElement;

    onMount(() => {
        console.log(preEl);
        // highlight.js のハイライト実行
        hljs.highlightElement(preEl);
    });
</script>

<pre bind:this={preEl} class="hljs linenumbers">
  <code class="language-{language}">
    {@html code}
  </code>
</pre>

<style>
    pre {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 4px;
        box-sizing: border-box;
        overflow: auto;
    }
    /*code {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 4px;
        box-sizing: border-box;
    }*/
</style>

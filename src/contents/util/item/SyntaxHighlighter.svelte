<script lang="ts">
    import { onMount } from "svelte";
    import hljs from "highlight.js";

    export let code: string = "";
    export let language: string = "javascript";

    let el: HTMLElement;

    onMount(() => {
        hljs.highlightElement(el);
    });
</script>

<pre><code bind:this={el} class="language-{language}">{code}</code></pre>

<style>
    /* 必要に応じてテーマを読み込む */
    @import "highlight.js/styles/github-dark.css";
</style>

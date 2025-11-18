<script lang="ts">
    import store, { createStoreUtil } from "../../store/store";
    import SyntaxHighlighter from "./SyntaxHighlighter.svelte";

    $: commit = createStoreUtil($store).commit;
    $: preview = (() => {
        if ($store.preview == undefined) throw new Error();
        return $store.preview;
    })();
</script>

<div class="wrap">
    <div
        class="inner"
        oncontextmenu={() => {
            delete $store.preview;
            commit();
        }}
    >
        <SyntaxHighlighter language={preview.language} code={preview.src} />
    </div>
</div>

<style>
    .wrap {
        display: inline-block;
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
    }
    .inner {
        display: inline-block;
        position: relative;
        margin: 4px 0 0 4px;
        width: calc(100% - 8px);
        height: calc(100% - 8px);
        background-color: #000;
        border: 1px solid #00a;
        box-sizing: border-box;
        font-size: 14px;
        /*color: #000;*/
    }
</style>

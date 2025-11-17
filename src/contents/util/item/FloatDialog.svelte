<script lang="ts">
    import SyntaxHighlighter from "./SyntaxHighlighter.svelte";
</script>

<div class="wrap">
    <div class="inner">
        <SyntaxHighlighter
            language="javascript"
            code={`count = res.counter;`}
        />
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
        background-color: #ffffffe1;
        border: 1px solid #00a;
        box-sizing: border-box;
    }
</style>

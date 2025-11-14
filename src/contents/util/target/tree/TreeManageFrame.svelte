<script lang="ts">
    import type { NodeDispProps } from "../../../store/types";
    import TreeItem from "./TreeItem.svelte";

    export let items!: NodeDispProps[];
</script>

<div>
    {#each items as item}
        <TreeItem name={item.str} indent={item.indent} />
    {/each}
</div>

<style>
    div {
        display: inline-block;
        position: relative;
        margin: 8px 0 0 8px;
        width: calc(100% - 16px);
        height: calc(100% - 16px);
        background-color: #ffffff;
        overflow: auto;
    }
</style>

<script lang="ts">
    import store, { createStoreUtil } from "../../../store/store";
    import type { NodeDispProps } from "../../../store/types";

    $: commit = createStoreUtil($store).commit;
    export let item!: NodeDispProps;

    $: toggleOpen = () => {
        item.node.isOpen = !item.node.isOpen;
        commit();
    };

    $: children = item.node.children;
</script>

<div class="wrap" style:margin-left="{item.indent * 10}px">
    {#if children != null && children.length > 0}
        <button onclick={toggleOpen}>{item.node.isOpen ? "-" : "+"}</button>
    {/if}
    <div class="str" data--file={item.node.children == null}>{item.str}</div>
</div>

<style>
    .wrap {
        display: block;
        position: relative;
        /*width: 200px;*/
        height: 24px;
        margin-top: 1px;
        background-color: #aaccff44;

        &:hover {
            background-color: #bbdd0011;
        }
    }
    button {
        display: inline-block;
        position: relative;
        width: 40px;
        height: calc(100% - 4px);
        background-color: #fff;
        box-sizing: border-box;
        border-radius: 2px;
        margin: 2px 0 0 4px;
        color: #222;
        font-size: 14px;
        font-weight: 400;
        border: 1px solid #338;
        text-align: center;
        &:hover {
            background-color: #ff5;
        }
    }
    .str {
        display: inline-block;
        position: relative;
        height: 100%;
        margin: 0 0 0 2px;
        font-size: 14px;
        color: #3333aa;
        &:hover {
            color: #33aa33;
        }
    }
    .str[data--file="true"] {
        background-color: #ff999999;
    }
</style>

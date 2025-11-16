<script lang="ts">
    import store, { createStoreUtil } from "../../../store/store";
    import type { NodeDispProps, UsableNode } from "../../../store/types";
    import TreeItem from "./TreeItem.svelte";

    export let root!: UsableNode;
    $: commit = createStoreUtil($store).commit;

    $: items = (() => {
        const list: NodeDispProps[] = [];

        const rec = (node: UsableNode, depth: number) => {
            list.push({
                indent: depth,
                str: node.name,
                node,
            });
            if (node.children != null && node.isOpen)
                node.children.forEach((c) => rec(c, depth + 1));
        };
        if (root.children == null) throw new Error();
        root.children.forEach((c) => rec(c, 0));

        console.log(list.length);
        return list;
    })();

    $: cancel = () => {
        delete $store.resultTree;
        commit();
    };
    $: transfer = () => {};
</script>

<div class="main">
    <div class="list">
        {#each items as item}
            <TreeItem {item} />
        {/each}
    </div>
</div>
<div class="operation-div">
    <button data--disable={false} onclick={cancel}>Cancel</button>
    <button data--disable={false} onclick={transfer}>Transfer</button>
</div>

<style>
    .main {
        display: inline-block;
        position: relative;
        width: 100%;
        height: calc(100% - 40px);
    }
    .list {
        display: inline-block;
        position: relative;
        margin: 8px 0 0 8px;
        width: calc(100% - 16px);
        height: calc(100% - 16px);
        background-color: #ffffff;
        overflow: auto;
    }
    button {
        display: inline-block;
        position: relative;
        width: 150px;
        height: 30px;
        background-color: #8888aa;
        box-sizing: border-box;
        border-radius: 4px;
        margin: 5px 4px 0 0;
        color: white;
        font-size: 18px;
        font-weight: 600;
        border: 1px solid #338;
        text-align: center;
        &:hover {
            opacity: 0.7;
        }
    }
    button[data--disable="true"] {
        opacity: 0.5;
        pointer-events: none;
    }
    .operation-div {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 40px;
        background-color: #8888aa44;
        text-align: right;
    }
</style>

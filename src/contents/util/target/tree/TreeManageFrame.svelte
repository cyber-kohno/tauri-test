<script lang="ts">
    import { writable } from "svelte/store";
    import store, { createStoreUtil } from "../../../store/store";
    import type {
        ChildProps,
        NodeDispProps,
        NodeIndent,
        UsableNode,
    } from "../../../store/types";
    import OperationButton from "../../item/OperationButton.svelte";
    import TreeItem from "./TreeItem.svelte";
    import FloatDialog from "../../item/FloatDialog.svelte";

    export let root!: UsableNode;
    $: commit = createStoreUtil($store).commit;

    const confirmStr = writable<string | null>(null);

    $: items = (() => {
        const list: NodeDispProps[] = [];

        const rec = (
            node: UsableNode,
            indents: NodeIndent[],
            isOpen: boolean,
        ): [number, number] => {
            const record: NodeDispProps = {
                indents,
                node,
            };
            if (isOpen) {
                list.push(record);
            }
            let [fileCnt, selectCnt] = [0, 0];
            if (node.child != undefined) {
                const child = node.child;
                [child.fileCnt, child.selectCnt] = [0, 0];
                const nodes = node.child.nodes;
                nodes.forEach((n, i) => {
                    const nextIndents: NodeIndent[] = indents.slice();
                    // 自身がlastの場合、子要素はnoneにする
                    if (nextIndents[nextIndents.length - 1] === "last")
                        nextIndents[nextIndents.length - 1] = "none";
                    nextIndents.push(
                        (() => {
                            if (i === nodes.length - 1) return "last";
                            else return "middle";
                        })(),
                    );
                    const [cFileCnt, cSelectCnt] = rec(
                        n,
                        nextIndents,
                        isOpen && child.isOpen,
                    );
                    child.fileCnt += cFileCnt;
                    child.selectCnt += cSelectCnt;
                });
                fileCnt += child.fileCnt;
                selectCnt += child.selectCnt;
            } else {
                fileCnt++;
                selectCnt += node.isSelected ? 1 : 0;
            }
            return [fileCnt, selectCnt];
        };
        rec(root, [], true);

        console.log(list.length);
        return list;
    })();

    $: cancel = () => {
        $confirmStr = "abc";
    };
    $: transfer = () => {};
</script>

<div class="operation-div">
    <OperationButton
        name={"Cancel"}
        width={160}
        disable={false}
        callback={cancel}
    />
</div>
<div class="main">
    <div class="list">
        {#each items as item}
            <TreeItem {item} />
        {/each}
    </div>
</div>
<div class="operation-div">
    <OperationButton
        name={"Cancel"}
        width={140}
        disable={false}
        callback={cancel}
    />
    <OperationButton
        name={"Transfer"}
        width={160}
        disable={false}
        callback={transfer}
    />
</div>
{#if $confirmStr != null}
    <FloatDialog />
{/if}

<style>
    .main {
        display: inline-block;
        position: relative;
        width: 100%;
        height: calc(100% - 64px);
    }
    .list {
        display: inline-block;
        position: relative;
        margin: 4px 0 0 4px;
        width: calc(100% - 8px);
        height: calc(100% - 8px);
        background-color: #ffffff;
        overflow: auto;
    }
    .operation-div {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 32px;
        background-color: #8888aa44;
        text-align: right;
    }
</style>

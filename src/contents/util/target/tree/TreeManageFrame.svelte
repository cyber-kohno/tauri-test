<script lang="ts">
    import store, { createStoreUtil } from "../../../store/store";
    import type {
        ChildProps,
        NodeDispProps,
        NodeIndent,
        UsableNode,
    } from "../../../store/types";
    import TreeItem from "./TreeItem.svelte";

    export let root!: UsableNode;
    $: commit = createStoreUtil($store).commit;

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

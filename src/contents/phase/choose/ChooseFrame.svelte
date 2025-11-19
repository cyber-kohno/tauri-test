<script lang="ts">
    import ChooseRecord from "./ChooseRecord.svelte";
    import { writable } from "svelte/store";
    import ChooseUtil from "./chooseUtil";
  import type { NodeDispProps, UsableNode } from "../../store/types";
  import store from "../../store/store";
  import OperationButton from "../../util/OperationButton.svelte";
  import FloatDialog from "../../util/FloatDialog.svelte";

    export let root: UsableNode;
    
    let ref: HTMLDivElement | undefined = undefined;

    let scrollTop = 0;

    const isFlat = writable<boolean>(false);

    $: baseRecords = (() => {
        const list = ChooseUtil.getDispRecords(root, $isFlat);
        console.log(list.length);
        return list;
    })();
    $: dispRecords = (() => {
        const list: NodeDispProps[] = [];
        baseRecords.forEach((r, i) => {
            if (ref != undefined) {
                const rect = ref.getBoundingClientRect();
                console.log(scrollTop);
                const criteria = -scrollTop + i * 25;
                console.log(criteria);
                if (criteria >= 0 && criteria <= rect.height) list.push(r);
            }
        });
        return list;
    })();

    $: cancel = () => {
        $store.resultTree = undefined;
    };
    $: toggleView = () => {
        $isFlat = !$isFlat;
    };
    $: transfer = () => {
        $store.phase = 'revalidate';
        $store.rvReq.targets = dispRecords.map(r => r.node.path).join('\n');
    };

    $: getDir = (item: NodeDispProps) => {
        let ret: string | null = null;
        if ($isFlat) {
            ret = item.node.path
                .replace($store.scanRequest.rootPath, "")
                .replace(item.node.name, "");
        }
        return ret;
    };
</script>

<div class="operation-div">
    <OperationButton
        name={!$isFlat ? "Flat" : "Tree"}
        width={120}
        disable={false}
        callback={toggleView}
    />
    <OperationButton
        name={!$isFlat ? "|← →|" : "|→ ←|"}
        width={90}
        disable={false}
        callback={toggleView}
    />
</div>
<div class="main">
    <div
        class="list"
        bind:this={ref}
        onscroll={(e) => {
            scrollTop = e.currentTarget.scrollTop;
        }}
    >
        <div class="inner" style:height="{baseRecords.length * 25}px">
            {#each dispRecords as item, i}
                <ChooseRecord {item} dir={getDir(item)} />
            {/each}
        </div>
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
{#if $store.preview != undefined}
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
    .inner {
        display: inline-block;
        position: relative;
        width: 100%;
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

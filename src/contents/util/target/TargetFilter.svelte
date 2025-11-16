<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import DirectoryNameFilterConditions from "./DirectoryNameFilterConditions.svelte";
    import store, { createStoreUtil } from "../../store/store";
    import { writable } from "svelte/store";
    import TreeManageFrame from "./tree/TreeManageFrame.svelte";
    import type { Node, ScanResponse, UsableNode } from "../../store/types";

    let name = writable("");
    let count = writable<number>(-1);
    let isSearch = writable(false);
    let scalnningDispDir = writable<string[]>([]);

    $: commit = createStoreUtil($store).commit;
    $: req = $store.scanRequest;

    $: isRequestOk = (() => {
        return (
            req.rootPath.length > 0 &&
            !req.dirConds.reduce(
                (hasErr, c) => hasErr || c.pattern.length === 0,
                false,
            )
        );
    })();

    const start = async () => {
        $isSearch = true;
        // Rustからの進捗イベントを受信
        const unlisten = await listen<any>("progress", (event) => {
            const res = event.payload;
            // console.log(res.path);
            count = res.counter;
            const path: string = res.path;
            $scalnningDispDir = path
                .split("\\")
                .map((s, i) => `${"_".repeat(i)}${s}`);
            // list.push(res.name);
        });

        const doneUnlisten = await listen("progress_done", () => {
            unlisten(); // 解除
            doneUnlisten();
            console.log("complete!");
            $isSearch = false;
        });

        try {
            const res: ScanResponse = await invoke("start_long_task", { req });
            console.log(res);
            const rec = (n: Node): UsableNode => ({
                name: n.name,
                children:
                    n.children == null ? null : n.children.map((c) => rec(c)),
                isOpen: false,
                isSelected: false,
            });
            $store.resultTree = rec(res.node);
        } catch (e) {
            console.error("Error:", e);
            alert("指定したディレクトリが不正です。");
            $isSearch = false;
        }
    };
</script>

<div class="wrap">
    <div class="left">
        <!-- リクエストフレーム -->
        <div class="list-frame">
            <!-- ルートパス -->
            <div class="label-record">{"*target_root_path"}</div>
            <input
                class="root-dir"
                data--blank={req.rootPath === ""}
                value={req.rootPath}
                oninput={(e) => {
                    req.rootPath = e.currentTarget.value;
                    commit();
                }}
            />
            <!-- 期待値算出時の階層 -->
            <div class="label-record">{"*expected_div_depth"}</div>
            <input
                class="depth"
                type="number"
                min="0"
                max="50"
                value={req.expectedDepth}
                oninput={(e) => {
                    const v = e.currentTarget.value;
                    req.expectedDepth = Number.parseInt(v);
                }}
            />
            <!-- 走査階層の上限（どこまで深くスキャンするか） -->
            <div class="label-record">{"*limit_depth"}</div>
            <input
                class="depth"
                type="number"
                min="0"
                max="50"
                value={req.limitDepth}
                oninput={(e) => {
                    const v = e.currentTarget.value;
                    req.limitDepth = Number.parseInt(v);
                }}
            />
            <div class="label-record">
                {"*directory_name_filter_conditions"}
            </div>
            <DirectoryNameFilterConditions />
            <div class="label-record">{"*file_name_filter_conditions"}</div>
        </div>
        <div class="operation-div">
            <button class="button" onclick={start}>Reset</button>
            <button class="button" data--disable={!isRequestOk} onclick={start}
                >Scan</button
            >
        </div>
    </div>
    <div class="right">
        <!-- <textarea value={$result} readonly></textarea> -->
        {#if $store.resultTree != undefined}
            <TreeManageFrame root={$store.resultTree} />
        {/if}
    </div>
    {#if $isSearch}
        <div class="blind">
            <div class="list-item">{count}</div>
            {#each $scalnningDispDir as a}
                <div class="list-item"><span>{a}</span></div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .wrap {
        display: inline-block;
        position: relative;
        background-color: #fec;
        width: 100%;
        height: 100%;
        > .left,
        .right {
            display: inline-block;
            position: relative;
            vertical-align: top;
            width: 50%;
            height: 100%;
        }
    }
    .left {
        background-color: #eff;
    }
    .operation-div {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 40px;
        background-color: #8888aa44;
        text-align: right;
    }
    .button {
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
    .button[data--disable="true"] {
        opacity: 0.5;
        pointer-events: none;
    }
    .right {
        background-color: #eff;
    }
    .list-frame {
        display: inline-block;
        position: relative;
        width: 100%;
        height: calc(100% - 40px);
        overflow: auto;
    }
    .list-item {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 20px;
        background-color: #ffffff33;
        font-size: 14px;
        color: #000000aa;
        padding: 0 0 0 4px;
        box-sizing: border-box;
        margin: 1px 0 0 0;
        /*overflow: hidden;*/
        white-space: nowrap;
    }
    .blind {
        display: inline-block;
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
        background-color: #ffffffcc;
        z-index: 2;
    }
    span {
        color: red;
    }
    .label-record {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 30px;
        background-color: #8888aa22;
        box-sizing: border-box;
        margin: 5px 0 0 0;
        color: #006;
        font-size: 18px;
        font-weight: 600;
        text-align: left;
        padding: 0 0 0 4px;
    }
    textarea {
        display: inline-block;
        position: relative;
        width: 100%;
        height: 100%;
        resize: none;
        background-color: #fff;
        outline: none;
        box-sizing: border-box;
        border: none;
        font-size: 18px;
        color: #000;
    }
    input {
        display: inline-block;
        position: relative;
        margin: 4px 0 0 4px;
        width: calc(100% - 8px);
        height: 20px;
        background-color: #fff;
        border: 1px solid #888;
        box-sizing: border-box;
        border-radius: 2px;
        font-size: 14px;
    }
    input[data--blank="true"] {
        background-color: #ff0;
    }
    input.root-dir {
        width: calc(100% - 8px);
    }
    input.depth {
        width: 80px;
    }
</style>

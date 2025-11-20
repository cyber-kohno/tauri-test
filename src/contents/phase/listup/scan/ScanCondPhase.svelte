<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import DirNameFilterConds from "./DirNameFilterConds.svelte";
  import { writable } from "svelte/store";
  import FileNameFilterConds from "./FileNameFilterConds.svelte";
  import store from "../../../store/store";
  import type {
    ChildProps,
    PayloadNode,
    ScanResponse,
    UsableNode,
  } from "../../../store/types";
  import NumberInput from "../../../util/form/NumberInput.svelte";
  import OperationButton from "../../../util/OperationButton.svelte";

  let count = writable<number>(-1);
  let isSearch = writable(false);
  let scalnningDispDir = writable<string[]>([]);

  $: req = $store.scanRequest;

  $: isRequestOk = () => {
    return (
      req.rootPath.length > 0 &&
      !req.dirConds.some((c) => c.pattern.length === 0) &&
      !req.fileConds.some((c) => c.pattern.length === 0)
    );
  };

  const reset = () => {
    req.rootPath = "";
    req.dirConds.length = 0;
    req.fileConds.length = 0;
    req.expectedDepth = 1;
    req.limitDepth = undefined;
    $store.scanRequest.dirConds = req.dirConds.slice();
    $store.scanRequest.fileConds = req.fileConds.slice();
  };

  /**
   * スキャン開始
   */
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
      const res: ScanResponse = await invoke("scan_directory", { req });
      // console.log(res);
      const rec = (n: PayloadNode, path: string): UsableNode => {
        const curPath = path + "\\" + n.name;
        let child: ChildProps | undefined = undefined;
        if (n.children != null) {
          const nodes = n.children.map((c) => rec(c, curPath));
          child = {
            fileCnt: 0,
            isOpen: false,
            selectCnt: 0,
            nodes,
          };
        }
        return {
          name: n.name,
          path: curPath,
          child,
          isSelected: false,
        };
      };
      $store.resultTree = rec(
        res.node,
        req.rootPath.split("\\").slice(0, -1).join("\\")
      );
    } catch (e) {
      console.error("Error:", e);
      alert("指定したディレクトリが不正です。");
      $isSearch = false;
    }
  };
</script>

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
      // commit();
    }}
  />
  <!-- 期待値算出時の階層 -->
  <div class="label-record">{"*expected_div_depth"}</div>
  <NumberInput
    min={0}
    max={50}
    value={req.expectedDepth}
    set={(v) => (req.expectedDepth = v)}
  />
  <!-- 走査階層の上限（どこまで深くスキャンするか） -->
  <div class="label-record">{"*limit_depth"}</div>
  <NumberInput
    min={0}
    max={50}
    value={req.limitDepth}
    set={(v) => (req.limitDepth = v)}
    isOptional
  />
  <!-- ディレクトリ名の抽出条件 -->
  <div class="label-record">
    {"*directory_name_filter_conditions"}
  </div>
  <DirNameFilterConds />
  <!-- ファイル名の抽出条件 -->
  <div class="label-record">{"*file_name_filter_conditions"}</div>
  <FileNameFilterConds />
</div>
<div class="operation-div">
  <OperationButton name={"Reset"} width={160} callback={reset} />
  <OperationButton
    name={"Scan"}
    width={160}
    disable={!isRequestOk()}
    callback={start}
  />
</div>
{#if $isSearch}
  <div class="blind">
    <div class="list-item">{count}</div>
    {#each $scalnningDispDir as a}
      <div class="list-item"><span>{a}</span></div>
    {/each}
  </div>
{/if}

<style>
  .operation-div {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 32px;
    background-color: #8888aa44;
    text-align: right;
  }
  .list-frame {
    display: inline-block;
    position: relative;
    width: 100%;
    height: calc(100% - 32px);
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
</style>

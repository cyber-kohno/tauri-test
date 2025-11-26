<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import store from "../../../store/store";
  import OperationButton from "../../../util/OperationButton.svelte";
  import MonacoEditor from "../../../util/MonacoEditor.svelte";

  $: req = (() => {
    const req = $store.executeReq;
    if (req == undefined) throw new Error();
    return req;
  })();

  $: cancel = () => {
    $store.executeReq = undefined;
    $store.phase = "revalidate";
  };
  $: execute = () => {
    console.log(`execute start! ${req.files.length}`);
    (async () => {
      // 一時的にコンソールログを変更
      const bak = console.log;
      console.log = (...args) => {
        req.output += `${args[0]}`;
        $store.executeReq = { ...req };
      };
      const func = new Function("content", req.funcSource);
      for (const f of req.files) {
        console.log(`${f}\n`);
        const content = await invoke("read_file", { req: { filePath: f } });
        func(content);
      }

      // コンソールログを戻す
      console.log = bak;
    })();
  };
</script>

<div class="main">
  <!-- <textarea
    value={req.funcSource}
    oninput={(e) => {
      req.funcSource = e.currentTarget.value;
    }}
  ></textarea> -->
  <div class="inner">
    <MonacoEditor
      bind:value={req.funcSource}
      language="javascript"
      theme="vs-dark"
      on:change={({ detail }) => {
        req.funcSource = detail.value;
      }}
    />
    {#if req.output !== ""}
      <div class="blind"></div>
    {/if}
  </div>
</div>
<div class="operation-div">
  <OperationButton name={"Cancel"} width={160} callback={cancel} />
  <OperationButton name={"Execute"} width={160} callback={execute} />
</div>

<style>
  .main {
    display: inline-block;
    position: relative;
    width: 100%;
    height: calc(100% - 32px);
    background-color: #2c6da2;
  }
  .inner {
    display: inline-block;
    position: relative;
    margin: 4px 0 0 4px;
    width: calc(100% - 8px);
    height: calc(100% - 8px);
  }
  .blind {
    display: inline-block;
    position: absolute;
    width: 100%;
    height: 100%;
    background-color: #2378227d;
    left: 0;
    top: 0;
    z-index: 12;
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

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
    // console.log(`execute start! ${req.files.length}`);

    $store.executeRes = {
      endCnt: 0,
      output: "",
    };
    const res = $store.executeRes;

    const start = async () => {
      const out = (str: string) => {
        res.output += `${str}`;
        $store.executeReq = { ...req };
      };
      const func = new Function("$out", "$path", "$content", req.funcSource);
      for (const f of req.files) {
        // console.log(`${f}\n`);
        const content = await invoke("read_file", { req: { filePath: f } });
        func(out, f, content);
        res.endCnt++;
        $store.executeRes = { ...res };
      }
    };

    start().catch((e) => {
      alert(e);
      $store.executeRes = undefined;
    });
  };

  $: isExecute = $store.executeRes != undefined;
</script>

<div class="main">
  <div class="inner">
    <MonacoEditor
      value={req.funcSource}
      language="typescript"
      theme="vs-dark"
      onChange={(value) => {
        req.funcSource = value;
      }}
      declares={[
        `declare const $out: (str: string) => void;`,
        `declare const $path: string;`,
        `declare const $content: string;`,
      ]}
    />
    {#if $store.executeRes != undefined}
      <div class="blind"></div>
    {/if}
  </div>
</div>
<div class="operation-div">
  <OperationButton
    name={"Cancel"}
    width={160}
    disable={isExecute}
    callback={cancel}
  />
  <OperationButton
    name={"Execute"}
    width={160}
    disable={isExecute || req.funcSource.length === 0}
    callback={execute}
  />
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

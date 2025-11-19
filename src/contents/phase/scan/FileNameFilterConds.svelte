<script lang="ts">
  import store from "../../store/store";
  import TextInput from "../../util/form/TextInput.svelte";

  $: req = $store.scanRequest;
  $: fileConds = req.fileConds;
  $: invalidate = () => ($store.scanRequest = {...req});

  /**
   * 終端に条件追加
   */
  const add = () => {
    fileConds.push({
      pattern: "",
      isExclusion: false,
    });
    invalidate();
  };

  /**
   * 指定行の条件削除
   */
  const del = (index: number) => {
    fileConds.splice(index, 1);
    invalidate();
  };
</script>

{#each fileConds as con, i}
  <div class="record">
    <button class="adddel" onclick={() => del(i)}>{"-"}</button>

    <!-- 含む・含まない（除外） -->
    <button
      class="toggle"
      data--flg={con.isExclusion}
      onclick={() => (con.isExclusion = !con.isExclusion)}
      >{con.isExclusion ? "-exclusion" : "+include"}</button
    >
    <!-- 正規表現パターン -->
    <TextInput
      value={con.pattern}
      width="calc(100% - 230px)"
      set={(v) => {
        con.pattern = v;
        invalidate();
      }}
      isRequied
    />
  </div>
{/each}
<div class="record"><button class="adddel" onclick={add}>{"+"}</button></div>

<style>
  .record {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 30px;
    /*background-color: #ffff0055;*/
    box-sizing: border-box;
    margin: 5px 0 0 0;
    color: #006;
    font-size: 18px;
    font-weight: 600;
    text-align: left;
    padding: 0 0 0 4px;
  }
  button {
    display: inline-block;
    position: relative;
    height: 26px;
    background-color: #eeeeee;
    box-sizing: border-box;
    border-radius: 4px;
    margin: 2px 0 0 4px;
    color: #000;
    font-size: 14px;
    /*font-weight: 600;*/
    border: 1px solid #888;
    box-sizing: border-box;
    text-align: center;
    &:hover {
      opacity: 0.7;
    }
  }
  button.adddel {
    width: 40px;
  }
  button.toggle {
    width: 80px;
  }
  button.toggle[data--flg="true"] {
    background-color: #ccf;
  }
</style>

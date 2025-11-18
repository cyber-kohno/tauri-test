<script lang="ts">
    import store, { createStoreUtil } from "../../store/store";

    $: commit = createStoreUtil($store).commit;
    $: dirConds = $store.scanRequest.dirConds;

    /**
     * 終端に条件追加
     */
    const add = () => {
        dirConds.push({
            pattern: "",
            isExclusion: false,
        });
        commit();
    };

    /**
     * 指定行の条件削除
     */
    const del = (index: number) => {
        dirConds.splice(index, 1);
        commit();
    };
</script>

{#each dirConds as con, i}
    <div class="record">
        <button class="adddel" onclick={() => del(i)}>{"-"}</button>
        <!-- 階層 -->
        <input
            class="depth"
            type="number"
            min="0"
            max="50"
            value={con.depth ?? ""}
            oninput={(e) => {
                const v = e.currentTarget.value;
                con.depth = v === "" ? undefined : Number.parseInt(v);
                commit();
            }}
        />
        <!-- 含む・含まない（除外） -->
        <button
            class="toggle"
            data--flg={con.isExclusion}
            onclick={() => (con.isExclusion = !con.isExclusion)}
            >{con.isExclusion ? "-exclusion" : "+include"}</button
        >
        <!-- 正規表現パターン -->
        <input
            class="patt"
            data--blank={con.pattern.length === 0}
            value={con.pattern}
            oninput={(e) => {
                con.pattern = e.currentTarget.value;
                $store.scanRequest.dirConds = dirConds.slice();
            }}
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
    input {
        display: inline-block;
        position: relative;
        margin: 5px 0 0 2px;
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
    input.patt {
        width: calc(100% - 250px);
    }
    input.depth {
        width: 80px;
    }
</style>

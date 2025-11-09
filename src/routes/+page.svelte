<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import "../app.css";
    import { listen } from "@tauri-apps/api/event";

    let name = $state("");
    let greetMsg = $state<string>("");

    async function greet(event: Event) {
        event.preventDefault();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        greetMsg = await invoke("greet", { name });
    }
    let progress = -1;

    const start = async () => {
        progress = 0;

        // Rustからの進捗イベントを受信
        const unlisten = await listen<number>("progress", (event) => {
            progress = event.payload;
            console.log(progress);
        });

        const doneUnlisten = await listen("progress_done", () => {
            unlisten(); // 解除
            doneUnlisten();
            console.log("complete!");
        });

        // Rustの処理開始
        await invoke("start_long_task");
    };
</script>

<div class="wrap">
    <div class="left">
        <div class="button" onclick={start}>Start</div>
    </div>
    <div class="right"></div>
</div>

<style>
    .wrap {
        display: inline-block;
        position: relative;
        background-color: #fc3;
        width: 100%;
        height: 100%;
        * {
            width: 50%;
            height: 100%;
        }
    }
    .left {
        background-color: #cca;
    }
    .right {
        background-color: #acc;
    }
    .button {
        display: inline-block;
        position: relative;
        width: 200px;
        height: 30px;
        background-color: #339;
        font-size: 18px;
    }
</style>

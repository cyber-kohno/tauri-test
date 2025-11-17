<script lang="ts">
    export let name: string = "dummy";
    export let callback: () => void = () => {};
    export let disable: boolean = false;
    export let width: number = 160;
</script>

<button data--disable={disable} style:width="{width}px" onclick={callback}
    >{name}</button
>

<style>
    button {
        display: inline-block;
        position: relative;
        height: 26px;
        line-height: 24px;
        background-color: #8888aa;
        box-sizing: border-box;
        border-radius: 4px;
        margin: 3px 4px 0 0;
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
</style>

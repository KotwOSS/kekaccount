<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let allow_hide: boolean;
    export let get_content: (hidden: boolean)=>string;
    export let test_content: (content: string)=>string;

    export let hidden: boolean = allow_hide;

    let input;

    let changed: boolean = false;
    let focused: boolean = false;
    let content: string;
    let error: string;

    $: content = get_content(hidden);

    function on_input(e) {
        error = test_content(input.value);
        changed = input.value !== content;
    }

    function save() {
        dispatch("save", {
            content: input.value
        });
    }

    function submit(e) {
        e.preventDefault();
        if(changed && !error) save();
    }

    function hide() {
        focused=false;
        if(!changed) hidden=true;
    }

    function show() {
        focused=true;
        hidden=false;
    }

    function toggle_hide() {
        if(!focused) hidden = !hidden;
    }
</script>

<form on:submit={submit} class="setting">
    {#if error}<p class="error break">{error}</p>{/if}
    
    <div>
    <input bind:this={input} type="text" value={content} on:blur={hide} on:focus={show} on:input={on_input}>    
    
    {#if changed}
        <button on:click={save} disabled={error!==undefined}>Save</button>
    {:else}
        {#if allow_hide}
        <button on:click={toggle_hide}>{hidden?"Show":"Hide"}</button>
        {/if}
    {/if}
    </div>
</form>


<style>
    input {
        width: 100%;
        text-align: left;
    }

    form>div {
        display: flex;
    }

    .error {
        color: red;
    }
</style>
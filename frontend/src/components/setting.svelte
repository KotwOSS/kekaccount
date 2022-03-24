<script lang="ts">
    // import { createEventDispatcher } from "svelte";
    
    // const dispatch = createEventDispatcher();

    export let content: string;
    export let clz: string;
    
    export let placeholder: string;
    export function set_content(content: string) {
        this.content = content;
    }
</script>

<input class={clz} type="text" placeholder={placeholder} value={content} on:input>
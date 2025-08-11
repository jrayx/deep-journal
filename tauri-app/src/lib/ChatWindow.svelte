<script lang="ts">
    import type { Chat } from './types';
    export let currentChat: Chat | null = null;
</script>

<div>
    {#if currentChat}
        <h2>{currentChat.title}</h2>
    {/if}
</div>
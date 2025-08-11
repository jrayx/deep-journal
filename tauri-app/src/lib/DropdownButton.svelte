<!-- DropdownButton.svelte -->
<script lang="ts">
  export let options: { label: string, action: () => void }[] = [
    { label: 'Rename', action: renameChat },
    { label: 'Delete', action: deleteChat }
  ];
  export let icon: string = '...';
  export let chat: Chat | null = null;
  let open = false;
  let dropdownRef: HTMLDivElement;
  
  import { onMount, onDestroy } from 'svelte';
  import { invokeUpdateChatTitle, invokeDeleteChat } from './api';
  import { bus } from './bus';
  
  onMount(() => {
    window.addEventListener('mousedown', handleClick);
  });

  onDestroy(() => {
    window.removeEventListener('mousedown', handleClick);
  });

  function handleOption(action: () => void) {
    action();
    open = false;
  }
  
  function handleClick(event: MouseEvent) {
    if (open && dropdownRef && !dropdownRef.contains(event.target as Node)) {
      open = false;
    }
  }
  
  async function renameChat() {
      bus.emit('rename-chat', {chatId: chat.id});
  }

  async function deleteChat() {
    let result = await invokeDeleteChat(chat.id);
    bus.emit('chat-deleted', { chatId: chat.id });
  }

</script>

<style>
.dropdown-btn-container {
  position: relative;
  display: inline-block;
}
.dropdown-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.5rem;
}
.dropdown-list {
  position: absolute;
  right: 0;
  top: 2.2rem;
  background: #1a1a1a;
  border: 1px solid #ccc;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  z-index: 10;
  min-width: 120px;
}
.dropdown-item {
  padding: 0.5rem 1rem;
  cursor: pointer;
}
.dropdown-item:hover {
  background: #2a2a2a;
}
</style>

<div class="dropdown-btn-container" bind:this={dropdownRef}>
  <button class="dropdown-btn" onclick={() => open = !open}>{icon}</button>
  {#if open}
    <div class="dropdown-list">
      {#each options as opt}
        <div class="dropdown-item" onclick={() => handleOption(opt.action)}>
          {opt.label}
        </div>
      {/each}
    </div>
  {/if}
</div>
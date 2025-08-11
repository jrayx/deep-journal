<script lang="ts">
    // inputs
    export let chats: Chat[] = [];
    let currentChat: Chat | null = null;
    
    
    import type { Chat } from './types';
    import { bus } from './bus';
    import { invokeCreateChat, invokeUpdateChatTitle } from './api';
    import DropdownButton from './DropdownButton.svelte';
    
    let renamingChatId: number | null = null;
    let renameValue: string = "";

    const onClickCreateNewChat = () => {
        // console.log("Create New Chat clicked");
        invokeCreateChat().then(newChat => {
            bus.emit('new-chat-created', newChat);
        });
    }
    
    const onClickChat = (chat: Chat) => {
        bus.emit('chat-selected', chat);
    }

    bus.on('rename-chat', ({ chatId }) => {
        renamingChatId = chatId;
        const chatToRename = chats.find(c => c.id === chatId);
        renameValue = chatToRename ? chatToRename.title : "";
    });
    
    async function submitRenameChat(chat: Chat) {
        let result = await invokeUpdateChatTitle(chat.id, renameValue);
        bus.emit('chat-renamed', { chatId: chat.id, newTitle: renameValue });
        renamingChatId = null;
    }

</script>

<style>
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  width: 300px;
  height: 100vh;
  background: #222;
  color: #fff;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 8px rgba(0,0,0,0.08);
  z-index: 100;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  padding: 1.5rem 1rem;
  border-bottom: 1px solid #333;
  letter-spacing: 2px;
}

.menu {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem;
}

.menu-header {
    font-weight: bold;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #333;
    text-align: left;
}

.menu-item {
  padding: 0.75rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.2s;
  text-align: left;
}
.menu-item:hover {
  background: #333;
}
.menu-item.selected {
  background: #2a2a2a;
  color: #fff;
}

</style>

<div class="sidebar">
  <div class="logo">deep-journal</div>
  <nav class="menu">
    <div class="menu-item">
        <button onclick={onClickCreateNewChat}>[+] Create New Chat</button>
    </div>
    <div class="menu-header">Chats</div>
    {#each chats as chat, i}
        <button type="button" class="menu-item {chat?.id === currentChat?.id ? 'selected': ''}" onclick={() => onClickChat(chat)}>
            {#if renamingChatId === chat.id}
            <input
                type="text"
                bind:value={renameValue}
                onblur={() => submitRenameChat(chat)}
                onkeydown={(e) => e.key === 'Enter' && submitRenameChat(chat)}
            />
            {:else}
            <span>{chat.title}</span>
            {/if}
            <DropdownButton icon="..." chat={chat} />
        </button>
    {/each}
  </nav>
</div>
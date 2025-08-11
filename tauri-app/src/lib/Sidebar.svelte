<script lang="ts">
    // inputs
    export let chats: Chat[] = [];
    
    import type { Chat } from './types';
    import { invokeCreateChat } from './api';
    import { bus } from './bus';

    const onClickCreateNewChat = () => {
        // console.log("Create New Chat clicked");
        invokeCreateChat().then(newChat => {
            bus.emit('new-chat-created', newChat);
        });
    }

    const onClickChat = (chat: Chat) => {
        bus.emit('chat-selected', chat);
    }
</script>

<style>
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  width: 220px;
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

.menu-header {
    font-weight: bold;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #333;
    text-align: left;
}


</style>

<div class="sidebar">
  <div class="logo">deep-journal</div>
  <nav class="menu">
    <div class="menu-item">
        <button onclick={onClickCreateNewChat}>[+] Create New Chat</button>
    </div>
    <div class="menu-header">Chats</div>
    {#each chats as chat}
        <button onclick={() => onClickChat(chat)}>
            <div class="menu-item">{chat.title}</div>
        </button>
    {/each}
  </nav>
</div>
<script lang="ts">
  // When using the Tauri API npm package:
  import { invoke } from '@tauri-apps/api/core';
  // const invoke = window.__TAURI__.core.invoke;
  
  // Import app components
  // import Counter from './lib/Counter.svelte'
  // import TestInvokeTauri from './lib/TestInvokeTauri.svelte'
  import Sidebar from './lib/Sidebar.svelte'
  import ChatWindow from './lib/ChatWindow.svelte';
  
  // manage state
  let currentChat: Chat | null = null;
  let chats: Chat[] = [];

  import { onMount } from 'svelte';
  import type { Model, Chat, Message } from './lib/types';
  import { invokeLLM, invokeGetChats, invokeCreateChat } from './lib/api';
  import { bus } from './lib/bus';
  
  
  onMount(async () => {
    // set up event handlers
    bus.on('new-chat-created', (newChat) => {
      chats = [newChat, ...chats];
      currentChat = newChat;
    });
    
    bus.on('chat-selected', (chatSelected) => {
      currentChat = chatSelected;
    });

    // pull existing chats
    chats = await invokeGetChats();

    // if existing chats, set current chat to most recent chat
    if (chats.length > 0) {
      currentChat = chats[0];
    } else {
      // if no existing chats, create one and set that to current chat
      let newChat = await invokeCreateChat();
      chats = [newChat, ...chats];
      currentChat = newChat;
      // console.log(chats);
    }
  });

  

</script>


<main class="layout">
  <div class="sidebar">
    <Sidebar chats={chats} />
  </div>
  <div class="main">
    <!-- <div class="card">
      <TestInvokeTauri />
    </div> -->
    <ChatWindow currentChat={currentChat} />
  </div>
</main>

<style>
  .layout {
  display: flex;
  height: 100vh;
  }
  .sidebar {
    width: 220px;
  }
  .main {
    flex: 1;
  }
  /* .card {
    padding: 2em;
    border: 1px solid #eee;
    border-radius: 8px;
    background: white;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  } */
</style>

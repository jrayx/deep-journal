<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Model, Chat, Message } from './lib/types';
  import { onMount } from 'svelte';
  import { invokeGetModels, invokeGetChats, invokeCreateChat, invokeLLM } from './lib/api';
  import { bus } from './lib/bus';
  
  // Import app components
  // import Counter from './lib/Counter.svelte'
  // import TestInvokeTauri from './lib/TestInvokeTauri.svelte'
  import Sidebar from './lib/Sidebar.svelte'
  import ChatWindow from './lib/ChatWindow.svelte';
  
  // manage state
  let currentChat: Chat | null = null;
  let chats: Chat[] = [];
  let currentModel: Model | null = null;
  let models: Model[] = [];
  
  
  onMount(async () => {
    // set up event handlers
    bus.on('new-chat-created', (newChat) => {
      chats = [newChat, ...chats];
      currentChat = newChat;
    });
    
    bus.on('chat-selected', (chatSelected) => {
      currentChat = chatSelected;
    });
    
    // pull existing models and set default to first model
    models = await invokeGetModels();
    currentModel = models[0];

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
    <ChatWindow currentChat={currentChat} models={models} currentModel={currentModel} />
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
    height: 100vh;
    width: 100vw;
    display: flex;
  }
  /* .card {
    padding: 2em;
    border: 1px solid #eee;
    border-radius: 8px;
    background: white;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  } */
</style>

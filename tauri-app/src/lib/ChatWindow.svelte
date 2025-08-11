<script lang="ts">
    import type { Chat, Message } from './types';
    import { invokeGetMessages } from './api';
    import { Sender } from './constants';
    
    export let currentChat: Chat | null = null;
    export let models: Model[] = [];
    export let currentModel: Model | null = null;

    let messages: Message[] = [];

    function onCurrentChatChange(chat) {
        if (!currentChat) {
            return
        }
        // console.log('Current chat changed:', chat);
        loadMessages();
    }
    
    function loadMessages() {
        console.log("Loading messages...")
        invokeGetMessages(currentChat!.id)
        .then((fetchedMessages: Message[]) => {
            messages = fetchedMessages;
            // console.log('Loaded messages:', messages);
        })
        .catch((error) => {
            console.error('Failed to load messages:', error);
        });
    }

    $: onCurrentChatChange(currentChat);
</script>

<style>
.chat-window {
  border-radius: 12px;
  padding: 1rem;
  height: 400px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  flex: 1;
  height: 100%;
  width: 100%;
}

.model-dropdown {
  position: absolute;
  top: 1rem;
  right: 1rem;
  opacity: 0.6;
  pointer-events: none; /* Prevent interaction */
}

.message {
  max-width: 70%;
  padding: 0.75rem 1rem;
  border-radius: 16px;
  font-size: 1rem;
  word-break: break-word;
}

.message.user {
  align-self: flex-end;
  background: #535bf2;
  color: #fff;
}

.message.system {
  align-self: flex-start;
  /* background: #e0e0e0; */
  color: #fff;
}
</style>


<div class="chat-window">
    <select class="model-dropdown" disabled>
        {#each models as model}
            <option selected={model.id === currentModel?.id}>{model.name}</option>
        {/each}
    </select>
    {#each messages as message}
    <div class="message {message.sender === Sender.User ? 'user' : 'system'}">
        {message.text}
    </div>
    {/each}
</div>
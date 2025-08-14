<script lang="ts">
    import { onMount, tick } from 'svelte';
    import type { Chat, Message } from './types';
    import { invokeGetMessages, invokeCreateMessage, invokeLLM } from './api';
    import { Sender } from './constants';
    
    // inputs
    export let currentChat: Chat | null = null;
    export let models: Model[] = [];
    export let currentModel: Model | null = null;
    
    // state
    let inputMessage: string = '';
    let messages: Message[] = [];
    let chatWindow: HTMLDivElement;
    let sending = false;

    const modelThinkingDelimiter = "...done thinking.";
    
    onMount(async () => {
      await tick();
      scrollToBottom();
    });

    // scroll to bottom whenever chat messages changes
    $: if (messages) {
        scrollToBottom();
    }
    
    async function scrollToBottom() {
        await tick();
        if (chatWindow) {
        chatWindow.scrollTop = chatWindow.scrollHeight;
        }
    }
    
    // trigger onCurrentChatChange to run every time currentChat changed
    $: onCurrentChatChange(currentChat);
    

    function onCurrentChatChange(chat) {
        if (!currentChat) {
            return
        }
        // console.log('Current chat changed:', chat);
        loadMessages();
    }
    
    
    
    function loadMessages() {
        // console.log("Loading messages...")
        invokeGetMessages(currentChat!.id)
        .then((fetchedMessages: Message[]) => {
            messages = fetchedMessages;
            // console.log('Loaded messages:', messages);
        })
        .catch((error) => {
            console.error('Failed to load messages:', error);
        });
    }
    
    function getThinkingFromMessageText(messageText) {
      if (!messageText.includes(modelThinkingDelimiter)) {
        return ''
      }
      return messageText.split(modelThinkingDelimiter)[0];
    }
    
    function getAnswerFromMessageText(messageText) {
      if (!messageText.includes(modelThinkingDelimiter)) {
        return messageText;
      }
      return messageText.split(modelThinkingDelimiter)[1];
    }
    
    async function sendMessage() {
        sending = true;
        let inputMessageObj: Message = {
            chat_id: currentChat!.id,
            model_id: currentModel!.id,
            text: inputMessage,
            sender: Sender.User
        };
        // console.log(inputMessageObj);
        let newUserMessage = await invokeCreateMessage(inputMessageObj);
        messages = [...messages, newUserMessage];
        inputMessage = '';
        
        // Add system response message
        let llmResponse = await invokeLLM(currentModel!.name, currentChat?.id);
        let llmResponseMsgObj = {
            chat_id: currentChat!.id,
            model_id: currentModel!.id,
            text: llmResponse,
            sender: Sender.System
        };
        // console.log(llmResponseMsgObj);
        let llmMessageCreated: Message = await invokeCreateMessage(llmResponseMsgObj);
        messages = [...messages, llmMessageCreated];
        sending = false;
    }
    
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
  height: 95%;
  width: 100%;
}

.messages-container {
  display: flex;
  flex: 1 1 auto;
  overflow-y: auto;
  flex-direction: column;
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
  text-align: left;
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

.input-bar {
  display: flex;
  gap: 0.5rem;
  padding: 1rem;
  /* border-top: 1px solid #eee; */
  align-items: center;
}

.input-bar input[type="text"] {
  flex: 1;
  padding: 0.75rem;
  border-radius: 6px;
  border: 1px solid #ccc;
  font-size: 1rem;
}

.input-bar button {
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  border: none;
  background: #535bf2;
  color: #fff;
  font-weight: bold;
  cursor: pointer;
  transition: background 0.2s;
}
.input-bar button:hover {
  background: #005bb5;
}

.thinking {
  color: #888;
  font-style: italic;
}

.spinner {
  width: 24px;
  height: 24px;
  border: 2px solid #eee;
  border-top: 2px solid #222;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin: 0 auto;
  background: transparent;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>


<div bind:this={chatWindow} class="chat-window">
    <h2>{currentChat?.title}</h2>
    
    <select class="model-dropdown" disabled>
        {#each models as model}
            <option selected={model.id === currentModel?.id}>{model.name}</option>
        {/each}
    </select>
    
    <div class="messages-container">
      {#each messages as message}
          <div class="message {message.sender === Sender.User ? 'user' : 'system'}">
          {#if message.sender === Sender.User}
              
                  {message.text}
          {:else}
              <p class="thinking">{getThinkingFromMessageText(message.text)}</p>
              <p>{getAnswerFromMessageText(message.text)}</p>
          {/if}
          </div>
      {/each}
    </div>
    
    <div class="input-bar">
        {#if sending}
          <div class="spinner"></div>
        {/if}
        <input
        type="text"
        bind:value={inputMessage}
        placeholder="Type your message..."
        on:keydown={(e) => e.key === 'Enter' && sendMessage()}
        />
        <button on:click={sendMessage}>Send</button>
    </div>
</div>
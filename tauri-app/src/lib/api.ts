import type { Model, Chat, Message } from './types';
import { Sender } from './constants';
import { invoke } from '@tauri-apps/api/core';


export const invokeSetupDb = async () => {
    try {
        const response = await invoke('setup_database');
        return response;
    } catch (error) {
        console.error('Failed to invoke setup_database:', error);
        return null;
    }
};

export const invokeLLM = async (modelName: string, chatId: number) => {
    try {
        const response = await invoke('run_llm', { model_name: modelName, chat_id: chatId })
        return response;
    } catch (error) {
        console.error('Failed to invoke LLM:', error);
        return null;
    }
    
}

export const invokeGetModels = async () => {
    try {
        const response = await invoke<Model[]>('get_models');
        // console.log("Models:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to get models:', error);
        return null;
    }
};

export const invokeGetChats = async () => {
    try {
        const response = await invoke<Chat[]>('get_chats');
        // console.log("Chats:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to get chats:', error);
        return null;
    }
};

export const invokeCreateChat = async () => {
    try {
        const response = await invoke<Chat>('create_chat');
        // console.log("Created Chat:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to create chat:', error);
        return null;
    }
}

    
export const invokeUpdateChatTitle = async (chatId: number, newTitle: string) => {
    try {
        const response = await invoke<Chat>('update_chat_title', { chat_id: chatId, new_title: newTitle });
        // console.log("Updated Chat Title:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to update chat title:', error);
        return null;
    }
}

export const invokeDeleteChat = async (chatId: number) => {
    try {
        const response = await invoke<Chat>('delete_chat', { chat_id: chatId });
        // console.log("Deleted Chat:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to delete chat:', error);
        return null;
    }
}

export const invokeGetMessages = async (chatId: number) => {
    try {
        const response = await invoke<Message[]>('get_messages_by_chat', { chat_id: chatId });
        // console.log("Messages:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to get messages:', error);
        return null;
    }
}

export const invokeCreateMessage = async (newMessage: Message) => {
    try {
        const response = await invoke<Message>('create_message',
            {
                chat_id: newMessage.chat_id,
                model_id: newMessage.model_id,
                text: newMessage.text,
                sender: newMessage.sender
            }
        );
        // console.log("Created Message:");
        // console.log(response);
        return response;
    } catch (error) {
        console.error('Failed to create message:', error);
        return null;
    }
}


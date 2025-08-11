import type { Model, Chat, Message } from './types';
import { Sender } from './constants';
import { invoke } from '@tauri-apps/api/core';

export const invokeTauriHelloWorld = () => {
    invoke('hello_world', { invoke_message: 'Hello!' }).then((response) => {
        console.log(response);
    });
}

export const invokeTauriReadFile = () => {
    invoke('read_file').then((response) => {
        console.log(response);
    });
}

export const invokeLLM = () => {
    invoke('run_llm', { model_name: 'deepseek-r1:1.5b', message: 'Hello!' }).then((response) => {
        console.log(response);
    });
}

export const invokeGetModels = async () => {
    try {
        const response = await invoke<Model[]>('get_models');
        // console.log("Models:");
        // console.log(response);
    } catch (error) {
        console.error('Failed to get models:', error);
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

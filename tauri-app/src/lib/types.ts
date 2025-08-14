
export type Model = {
    id: number;
    name: string;
};

export type Chat = {
    id: number;
    title: string;
};

export type Message = {
    id: number;
    chat_id: number;
    model_id: number;
    text: string;
    sender: number;
}

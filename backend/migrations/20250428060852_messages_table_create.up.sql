-- Add up migration script here
CREATE TABLE IF NOT EXISTS messages (
    message_id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    chat_id uuid,
    user_id uuid,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    message_text text,
    FOREIGN KEY (chat_id) REFERENCES Chats(chat_id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users(user_id) ON UPDATE CASCADE ON DELETE CASCADE
);
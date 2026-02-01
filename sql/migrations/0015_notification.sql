CREATE TABLE notification (
    id VARCHAR(25) PRIMARY KEY,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    type VARCHAR(25) NOT NULL, -- 'event_invite', 'chat_invite', 'new_message', 'comment'
    
    -- Target entity
    event_id VARCHAR(25) REFERENCES event(id) ON DELETE CASCADE,
    chat_id VARCHAR(25) REFERENCES chat(id) ON DELETE CASCADE,
    post_id VARCHAR(25) REFERENCES post(id) ON DELETE CASCADE,
    journey_id VARCHAR(25) REFERENCES journey(id) ON DELETE CASCADE,
    
    -- Message
    title VARCHAR(256) NOT NULL,
    message TEXT NOT NULL,
    
    -- Status
    is_read BOOLEAN DEFAULT FALSE,
    
    ctime TIMESTAMPTZ DEFAULT now(),
    rtime TIMESTAMPTZ -- read time
);
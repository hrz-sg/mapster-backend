CREATE TABLE event_content(
    id VARCHAR(25) PRIMARY KEY,
    event_id VARCHAR(25) NOT NULL REFERENCES event(id) ON DELETE CASCADE,
    message_id VARCHAR(25) NOT NULL REFERENCES chat_message(id) ON DELETE CASCADE,

    post_id VARCHAR(25) REFERENCES post(id) ON DELETE CASCADE,
    journey_id VARCHAR(25) REFERENCES journey(id) ON DELETE CASCADE,
    
    suggested_by_user_id VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    is_selected BOOLEAN DEFAULT TRUE,
    sort_order INT DEFAULT 0,
    
    CHECK (post_id IS NOT NULL OR journey_id IS NOT NULL)
);

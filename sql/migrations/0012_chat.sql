-- CREATE TABLE chat_session (
--     id VARCHAR(25) PRIMARY KEY NOT NULL,
--     user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
--     logged_at TIMESTAMPTZ DEFAULT NOW()
-- );

CREATE TABLE chat(
    id VARCHAR(25) PRIMARY KEY,
    chat_type chat_type NOT NULL,
    title VARCHAR(256),
    owner_id VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    event_id VARCHAR(25) REFERENCES event(id) ON DELETE SET NULL,

    -- Timestamps
    ctime TIMESTAMPTZ DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE event ADD COLUMN chat_id VARCHAR(25) REFERENCES chat(id) ON DELETE SET NULL;

-- Chat Participant
CREATE TABLE chat_member(
    chat_id VARCHAR(25) NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT now(),
    left_at TIMESTAMPTZ,
    PRIMARY KEY (chat_id, user_id)
);

CREATE TABLE chat_message (
    id VARCHAR(25) PRIMARY KEY,
    chat_id VARCHAR(25) NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id), -- sender_id

    message_type message_type NOT NULL,

    text TEXT,

    post_id VARCHAR(25) REFERENCES post(id),
    journey_id VARCHAR(25) REFERENCES journey(id),

    reply_to_id VARCHAR(25) REFERENCES chat_message(id),

    ctime TIMESTAMPTZ DEFAULT now(),
    mtime TIMESTAMPTZ,
    -- dtime TIMESTAMPTZ,

    CHECK (
        (message_type = 'Text' AND text IS NOT NULL) OR
        (message_type = 'Post' AND post_id IS NOT NULL) OR
        (message_type = 'Journey' AND journey_id IS NOT NULL)
    )
);
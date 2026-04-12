CREATE TABLE chat(
    id VARCHAR(25) PRIMARY KEY,
    chat_type chat_type NOT NULL,
    direct_key VARCHAR(51),
    title VARCHAR(256),
    owner_id VARCHAR(25) REFERENCES "user"(id) ON DELETE SET NULL,
    event_id VARCHAR(25) REFERENCES event(id) ON DELETE SET NULL,

    -- Timestamps
    ctime TIMESTAMPTZ DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX chat_direct_key_unique ON chat(direct_key) WHERE direct_key IS NOT NULL;
ALTER TABLE event ADD COLUMN chat_id VARCHAR(25) REFERENCES chat(id) ON DELETE SET NULL;

CREATE TABLE chat_seq (
    chat_id VARCHAR(25) PRIMARY KEY,
    last_seq BIGINT NOT NULL DEFAULT 0
);

-- Chat Participant
CREATE TABLE chat_member(
    chat_id VARCHAR(25) NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    last_read_seq BIGINT NOT NULL DEFAULT 0, -- highest seq the user has acknowledged reading
    joined_at TIMESTAMPTZ DEFAULT now(),
    left_at TIMESTAMPTZ,
    PRIMARY KEY (chat_id, user_id)
);

CREATE TABLE chat_message (
    id VARCHAR(25) PRIMARY KEY,
    seq BIGINT NOT NULL, -- sort/clustering key: strict ordering within conversation
    chat_id VARCHAR(25) NOT NULL REFERENCES chat(id) ON DELETE CASCADE,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id), -- sender_id
    client_message_id VARCHAR(25), -- device sended id

    message_type message_type NOT NULL,

    text TEXT,

    post_id VARCHAR(25) REFERENCES post(id),
    journey_id VARCHAR(25) REFERENCES journey(id),

    reply_to_id VARCHAR(25) REFERENCES chat_message(id),

    ctime TIMESTAMPTZ DEFAULT now(),
    mtime TIMESTAMPTZ,
    dtime TIMESTAMPTZ
);

CREATE UNIQUE INDEX uniq_client_msg ON chat_message (chat_id, owner_id, client_message_id) WHERE client_message_id IS NOT NULL;
CREATE UNIQUE INDEX chat_message_chat_id_seq_idx ON chat_message (chat_id, seq);

-- Per-device delivery table: tracks delivery watermark per device (finer-grained than membership)
CREATE TABLE per_device_delivered (
    chat_id VARCHAR(25) NOT NULL,
    device_id VARCHAR(64) NOT NULL,        -- unique device identifier (one user may have many)
    last_delivered_seq BIGINT NOT NULL DEFAULT 0,  -- highest seq confirmed delivered to this device
    PRIMARY KEY (chat_id, device_id)
);

-- Helper index: look up all devices for a given conversation efficiently
CREATE INDEX idx_per_device_conversation ON per_device_delivered (chat_id);
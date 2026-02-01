CREATE TABLE event (
    id VARCHAR(25) PRIMARY KEY,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    location_id VARCHAR(25) REFERENCES location(id) ON DELETE SET NULL,
    title VARCHAR(256) NOT NULL,
    description TEXT,
    cover_media_url TEXT,
    status event_status NOT NULL DEFAULT 'Planning',
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,

    -- Timestamps
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE event_participant (
    event_id VARCHAR(25) NOT NULL REFERENCES event(id) ON DELETE CASCADE,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    role VARCHAR(25) NOT NULL DEFAULT 'participant',
    status VARCHAR(25) NOT NULL DEFAULT 'invited', -- 'invited', 'accepted', 'declined'

    -- Timestamps
    invited_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    responded_at TIMESTAMPTZ,
    
    PRIMARY KEY (event_id, user_id)
);
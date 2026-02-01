-- Post Likes
CREATE TABLE post_like (
    post_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    user_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (post_id, user_id)
);

CREATE INDEX idx_post_like_user ON post_like(user_id);

-- Post collection
CREATE TABLE post_collection (
    id VARCHAR(25) PRIMARY KEY,
    owner_id VARCHAR(25) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(128) NOT NULL,
    sort_order INT DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT FALSE, -- "All Posts"
    
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    UNIQUE(owner_id, title) -- unique names
);

CREATE INDEX idx_post_collection_owner ON post_collection(owner_id);

-- Junction table for post collections (allows multiple posts per collection)
CREATE TABLE post_collection_item (
    id VARCHAR(25) PRIMARY KEY,
    collection_id VARCHAR(25) NOT NULL REFERENCES post_collection(id) ON DELETE CASCADE,
    post_id VARCHAR(25) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    sort_order INT DEFAULT 0,
    
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    UNIQUE(collection_id, post_id)
);

CREATE INDEX idx_post_collection_item_post ON post_collection_item(post_id);
CREATE INDEX idx_post_collection_item_collection ON post_collection_item(collection_id);

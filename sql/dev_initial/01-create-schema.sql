---- Base app schema

-- User
CREATE TYPE user_typ AS ENUM ('Sys', 'User');

CREATE TABLE "user" (
    id VARCHAR(30) PRIMARY KEY,
    typ user_typ NOT NULL DEFAULT 'User',
    username VARCHAR(128) NOT NULL UNIQUE,
    email VARCHAR(256) NOT NULL UNIQUE,
    avatar_url TEXT,
    bio TEXT,
    location TEXT,

    -- Auth
    pwd VARCHAR(256),
    pwd_salt UUID NOT NULL DEFAULT gen_random_uuid(),

    -- Token
    token_salt UUID NOT NULL DEFAULT gen_random_uuid(),
    reset_token TEXT,
    reset_token_expires_at TIMESTAMPTZ,

    -- Email verification
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verification_token VARCHAR(255),
    email_verification_expires_at TIMESTAMPTZ,

    -- For soft delete
    deleted_at TIMESTAMPTZ,
    scheduled_permanent_deletion_at TIMESTAMPTZ,

    -- Timestamps / Audit
    cid VARCHAR(30) REFERENCES "user"(id),
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Followers & Followings
CREATE TABLE user_follow (
    follower_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    following_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (follower_id, following_id)
);

CREATE INDEX idx_user_follow_following ON user_follow(following_id);

-- Stats
CREATE TABLE user_stats (
    owner_id VARCHAR(30) PRIMARY KEY REFERENCES "user"(id) ON DELETE CASCADE,
    posts_count BIGINT NOT NULL DEFAULT 0,
    followers_count BIGINT NOT NULL DEFAULT 0,
    following_count BIGINT NOT NULL DEFAULT 0
);

-- Post
CREATE TABLE post (
    id VARCHAR(30) PRIMARY KEY,
    owner_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    cover_media_url TEXT,
    thumbnail_url TEXT,
    media_count INT NOT NULL DEFAULT 0,
    has_video BOOLEAN NOT NULL DEFAULT FALSE,
    like_count BIGINT NOT NULL DEFAULT 0,
    comment_count BIGINT NOT NULL DEFAULT 0,
    saved_count BIGINT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_post_owner_ctime ON post (owner_id, ctime DESC); -- User posts
CREATE INDEX idx_post_feed ON post (ctime DESC) WHERE is_published = true; -- public feed

-- Post Likes
CREATE TABLE post_like (
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (post_id, user_id)
);

CREATE INDEX idx_post_like_user ON post_like(user_id);

-- Post save
CREATE TABLE post_save (
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (post_id, user_id)
);

CREATE INDEX idx_post_save_user ON post_save(user_id);

-- Post collection
CREATE TABLE post_collection (
    id VARCHAR(30) PRIMARY KEY,
    owner_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(128) NOT NULL,
    sort_order INT DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT FALSE, -- "All Posts"
    
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    UNIQUE(owner_id, title) -- unique names
);

CREATE INDEX idx_post_collection_owner ON post_collection(owner_id);

CREATE TABLE post_collection_item (
    id VARCHAR(30) PRIMARY KEY,
    collection_id VARCHAR(30) NOT NULL REFERENCES post_collection(id) ON DELETE CASCADE,
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    
    -- Customization (available only for posts)
    custom_title VARCHAR(256),
    is_favorite BOOLEAN DEFAULT FALSE,
    
    sort_order INT DEFAULT 0,
    
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    UNIQUE(collection_id, post_id) -- пост в коллекции только 1 раз
);

CREATE INDEX idx_post_collection_item_post ON post_collection_item(post_id);
CREATE INDEX idx_post_collection_item_collection ON post_collection_item(collection_id);

-- Post Forward
CREATE TABLE post_forward (
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    forward_to_user_id VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (post_id, user_id, forward_to_user_id)
);

CREATE INDEX idx_post_forward_user ON post_forward(user_id);
CREATE INDEX idx_post_forward_to_user ON post_forward(forward_to_user_id);

-- PostMedia
CREATE TABLE post_media (
    id VARCHAR(30) PRIMARY KEY,
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(16) NOT NULL,
    mime_type VARCHAR(128) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    duration INT,
    sort_order INT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Comments
CREATE TYPE comment_entity_typ AS ENUM ('Post');

CREATE TABLE comment (
    id VARCHAR(30) PRIMARY KEY,
    owner_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    entity_type comment_entity_typ NOT NULL,
    entity_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    parent_id VARCHAR(30) REFERENCES comment(id) ON DELETE CASCADE,
    text TEXT NOT NULL,

    -- Timestamps / Audit
    cid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_comment_entity ON comment(entity_type, entity_id); -- Comments to entities (Post)
CREATE INDEX idx_comment_parent ON comment(parent_id); -- Comment replies

-- CommentMedia
CREATE TABLE comment_media (
    id VARCHAR(30) PRIMARY KEY,
    comment_id VARCHAR(30) NOT NULL REFERENCES comment(id) ON DELETE CASCADE,
    owner_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    media_url TEXT NOT NULL,
    media_type VARCHAR(16) NOT NULL CHECK (media_type = 'image'),
    mime_type VARCHAR(128) NOT NULL,
    width INT,
    height INT,
    file_size BIGINT,
    sort_order INT NOT NULL DEFAULT 0,
    is_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_reason TEXT,

    -- Timestamps / Audit
    cid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now(),

    FOREIGN KEY (comment_id) REFERENCES comment(id) ON DELETE CASCADE
);

-- Journey
CREATE TABLE journey (
    id VARCHAR(30) PRIMARY KEY,
    owner_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    title VARCHAR(256) NOT NULL,
    description TEXT,
    cover_media_url TEXT,
    is_published BOOLEAN NOT NULL DEFAULT FALSE,

    -- Stats (cache)
    total_likes BIGINT NOT NULL DEFAULT 0, -- the sum of all likes from posts in journey
    saved_count BIGINT NOT NULL DEFAULT 0,
    forward_count BIGINT NOT NULL DEFAULT 0,

    -- Timestamps / Audit
    cid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mid VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_journey_owner ON journey(owner_id); -- user journeys
CREATE INDEX idx_journey_feed ON journey (ctime DESC) WHERE is_published = true; -- journeys for feed

-- Journey Saved
CREATE TABLE journey_save(
    journey_id VARCHAR(30) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE, -- user who saved journey
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (journey_id, user_id)
);

CREATE INDEX idx_journey_save_user ON journey_save(user_id); -- user's saved journeys

-- Journey Forward
CREATE TABLE journey_forward (
    journey_id VARCHAR(30) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    user_id VARCHAR(30) NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    forward_to_user_id VARCHAR(30) REFERENCES "user"(id) ON DELETE SET NULL,
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (journey_id, user_id, forward_to_user_id)
);

CREATE INDEX idx_journey_forward_user ON journey_forward(user_id); -- FROM user's forwards
CREATE INDEX idx_journey_forward_to_user ON journey_forward(forward_to_user_id); -- TO user's forwards

CREATE TABLE journey_post (
    journey_id VARCHAR(30) NOT NULL REFERENCES journey(id) ON DELETE CASCADE,
    post_id VARCHAR(30) NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    sort_order INT NOT NULL,

    PRIMARY KEY (journey_id, post_id),
    UNIQUE (journey_id, sort_order),
    UNIQUE (post_id),

    -- Timestamps
    ctime TIMESTAMPTZ NOT NULL DEFAULT now(),
    mtime TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create root user
INSERT INTO "user" 
    (id, typ, username, email, cid, ctime, mid, mtime) VALUES 
    ('usr_sys_root', 'Sys', 'root', 'root@system.com', 'usr_sys_root', now(), 'usr_sys_root', now());

-- Create demo0 User
INSERT INTO "user"
    (id, username, email, cid, ctime, mid, mtime) VALUES 
    ('usr_demo0', 'demo0', 'demo0@example.com', 'usr_sys_root', now(), 'usr_sys_root', now());

-- ==========================================================
-- USERS 
-- ==========================================================

-- Demo user 1
INSERT INTO "user"
    (id, typ, username, email, cid, ctime, mid, mtime)
VALUES
    ('usr_demo1', 'User', 'demo1', 'demo132132@test.com',
     'usr_sys_root', now(), 'usr_sys_root', now());

-- Demo user 2
INSERT INTO "user"
    (id, typ, username, email, cid, ctime, mid, mtime)
VALUES
    ('usr_demo2', 'User', 'kristina_23', 'kristina_23.love@test.com',
     'usr_sys_root', now(), 'usr_sys_root', now());

-- Demo user 3
INSERT INTO "user"
    (id, typ, username, email, cid, ctime, mid, mtime)
VALUES
    ('usr_demo3', 'User', 'john_funk', 'john_funk06@test.com',
     'usr_sys_root', now(), 'usr_sys_root', now());

INSERT INTO user_stats (owner_id, posts_count, followers_count, following_count)
VALUES
    -- Root
    ('usr_sys_root', 0, 0, 0),

    -- demo0
    ('usr_demo0', 5, 2, 3),

    -- demo1
    ('usr_demo1', 12, 5, 4),

    -- demo2
    ('usr_demo2', 8, 7, 6),

    -- demo3
    ('usr_demo3', 20, 10, 8);

INSERT INTO post (
    id,
    owner_id,
    location_id,
    title,
    description,
    status,
    cover_media_key,
    media_count,
    like_count,
    comment_count,
    save_count,
    forward_count
)
VALUES
(
    'pst_demo1',
    'usr_demo0',
    NULL,
    'Trip to Shanghai',
    'First day in Shanghai',
    'Published',
    'post/pst_demo1/cover.jpg',
    1,
    0,
    0,
    0,
    0
),
(
    'pst_demo2',
    'usr_demo0',
    NULL,
    'Walking in Beijing',
    'Exploring Beijing streets',
    'Published',
    'post/pst_demo2/cover.jpg',
    1,
    0,
    0,
    0,
    0
),
(
    'pst_demo3',
    'usr_demo0',
    NULL,
    'Great Wall',
    'Amazing view from the Great Wall',
    'Published',
    'post/pst_demo3/cover.jpg',
    1,
    0,
    0,
    0,
    0
),
(
    'pst_demo4',
    'usr_demo0',
    NULL,
    'Post 4',
    'Amazing view post 4',
    'Published',
    'post/pst_demo4/cover.jpg',
    1,
    0,
    0,
    0,
    0
);
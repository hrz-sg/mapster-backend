-- Create root user
INSERT INTO "user" 
    (id, typ, username, email, cid, ctime, mid, mtime) VALUES 
    ('usr_sys_root', 'Sys', 'root', 'root@system.com', 'usr_sys_root', now(), 'usr_sys_root', now());

-- Create demo1 User
INSERT INTO "user"
    (id, username, email, avatar_url, cid, ctime, mid, mtime) VALUES 
    ('usr_demo0', 'demo0', 'demo0@example.com', 'https://images.unsplash.com/photo-1599566150163-29194dcaad36?q=80&w=387&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'usr_sys_root', now(), 'usr_sys_root', now());

-- ==========================================================
-- USERS 
-- ==========================================================

-- Demo user 1
INSERT INTO "user"
    (id, typ, username, email, avatar_url, cid, ctime, mid, mtime)
VALUES
    ('usr_demo1', 'User', 'demo1', 'demo132132@test.com',
     'https://images.unsplash.com/photo-1438761681033-6461ffad8d80?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
     'usr_sys_root', now(), 'usr_sys_root', now());

-- Demo user 2
INSERT INTO "user"
    (id, typ, username, email, avatar_url, cid, ctime, mid, mtime)
VALUES
    ('usr_demo2', 'User', 'kristina_23', 'kristina_23.love@test.com',
     'https://plus.unsplash.com/premium_photo-1670282393309-70fd7f8eb1ef?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8Z2lybHxlbnwwfHwwfHx8MA%3D%3D',
     'usr_sys_root', now(), 'usr_sys_root', now());

-- Demo user 3
INSERT INTO "user"
    (id, typ, username, email, avatar_url, cid, ctime, mid, mtime)
VALUES
    ('usr_demo3', 'User', 'john_funk', 'john_funk06@test.com',
     'https://images.unsplash.com/photo-1600486913747-55e5470d6f40?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTV8fG1hbnxlbnwwfHwwfHx8MA%3D%3D',
     'usr_sys_root', now(), 'usr_sys_root', now());


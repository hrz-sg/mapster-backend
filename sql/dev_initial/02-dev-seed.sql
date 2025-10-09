-- Create root user
INSERT INTO "user" 
    (id, typ, username, email, cid, ctime, mid, mtime) VALUES 
    (0, 'Sys', 'root', 'root@system.com', 0, now(), 0, now());

-- Create demo1 User
INSERT INTO "user" 
    (username, email, cid, ctime, mid, mtime) VALUES 
    ('demo1', 'demo1@example.com', 0, now(), 0, now());
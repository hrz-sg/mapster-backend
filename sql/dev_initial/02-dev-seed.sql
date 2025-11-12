-- Create root user
INSERT INTO "user" 
    (id, typ, username, email, cid, ctime, mid, mtime) VALUES 
    (0, 'Sys', 'root', 'root@system.com', 0, now(), 0, now());

-- Create demo1 User
INSERT INTO "user" 
    (username, email, cid, ctime, mid, mtime) VALUES 
    ('demo1', 'demo1@example.com', 0, now(), 0, now());

-- ==========================================================
-- USERS 
-- ==========================================================

-- Demo user 1
INSERT INTO "user"
    (id, typ, username, email, avatar_url, cid, ctime, mid, mtime)
VALUES
    (1001, 'User', 'stacy.up', 'stacy.up32132@test.com', 'https://images.unsplash.com/photo-1438761681033-6461ffad8d80?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 0, now(), 0, now());

-- Demo user 2
INSERT INTO "user"
    (id, typ, username, email, avatar_url, cid, ctime, mid, mtime)
VALUES
    (1002, 'User', 'kristina_23', 'kristina_23.love@test.com', 'https://plus.unsplash.com/premium_photo-1670282393309-70fd7f8eb1ef?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8Z2lybHxlbnwwfHwwfHx8MA%3D%3D', 0, now(), 0, now());

-- Demo user 3
INSERT INTO "user"
    (id, typ, username, email, avatar_url, cid, ctime, mid, mtime)
VALUES
    (1003, 'User', 'john_funk', 'john_funk06@test.com', 'https://images.unsplash.com/photo-1600486913747-55e5470d6f40?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTV8fG1hbnxlbnwwfHwwfHx8MA%3D%3D', 0, now(), 0, now());

-- ==========================================================
-- POSTS 
-- ==========================================================

-- Posts by User 1
INSERT INTO post (id, user_id, title, description, is_published, thumbnail_url, media_count, has_video, like_count, cid, ctime, mid, mtime)
VALUES
    (2001, 1001, 'My first trip to Shanghai', 'Photos and videos from my trip.', TRUE, 'https://images.unsplash.com/photo-1538428494232-9c0d8a3ab403?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 3, TRUE, 42, 0, now(), 0, now()),
    (2002, 1001, 'Morning hike', 'Sunrise hike in the mountains.', TRUE, 'https://images.unsplash.com/photo-1551632811-561732d1e306?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 2, FALSE, 12, 0, now(), 0, now()),
    (2003, 1001, 'Italy weekend', 'Enjoying weekend with my family in Italy', TRUE, 'https://images.unsplash.com/photo-1516483638261-f4dbaf036963?q=80&w=386&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 2, FALSE, 20, 0, now(), 0, now()),
    (2004, 1001, 'Beautiful Japan', 'Hanging out with my friends in Japan', FALSE, NULL, 1, FALSE, 5, 0, now(), 0, now());

-- Posts by User 2
INSERT INTO post (id, user_id, title, description, is_published, thumbnail_url, media_count, has_video, like_count, cid, ctime, mid, mtime)
VALUES
    (2005, 1002, 'Amazing Japan', 'Japan is the nicest place I ever seen', TRUE, 'https://plus.unsplash.com/premium_photo-1661964177687-57387c2cbd14?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 3, TRUE, 33, 0, now(), 0, now()),
    (2006, 1002, 'My day off', 'Flower fields outside. Look!', TRUE, 'https://images.unsplash.com/photo-1465146344425-f00d5f5c8f07?q=80&w=876&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 2, FALSE, 18, 0, now(), 0, now()),
    (2007, 1002, 'Urban Chicago', 'Nice place to visit', TRUE, 'https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?q=80&w=944&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 1, FALSE, 7, 0, now(), 0, now()),
    (2008, 1002, 'My trip to Moscow', 'From Europe to Moscow.', TRUE, 'https://images.unsplash.com/photo-1513326738677-b964603b136d?q=80&w=449&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 2, FALSE, 15, 0, now(), 0, now()),
    (2009, 1002, 'Mongolia horses', 'Exploring Mongolia', FALSE, NULL, 1, FALSE, 2, 0, now(), 0, now());

-- Posts by User 3
INSERT INTO post (id, user_id, title, description, is_published, thumbnail_url, media_count, has_video, like_count, cid, ctime, mid, mtime)
VALUES
    (2010, 1003, 'Holiday in Germany', 'Look at these gorgeous places', TRUE, 'https://plus.unsplash.com/premium_photo-1661962435210-e6cdbb2cbeb4?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 3, FALSE, 25, 0, now(), 0, now()),
    (2011, 1003, 'Business trip to France', 'Capturing France in motion.', TRUE, 'https://mapster-test-123.oss-cn-shanghai.aliyuncs.com/1112659-hd_720_720_25fps.mp4', 2, TRUE, 40, 0, now(), 0, now()),
    (2012, 1003, 'Spain is another amazing place to visit', 'Look how amazing Spain is', TRUE, 'https://plus.unsplash.com/premium_photo-1716138192476-f34e85ad43c2?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 1, FALSE, 11, 0, now(), 0, now()),
    (2013, 1003, 'Greece! Omg!', 'I spent a couple of weeks chilling out in Greece.', TRUE, 'https://images.unsplash.com/photo-1613395877344-13d4a8e0d49e?q=80&w=435&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 3, TRUE, 60, 0, now(), 0, now());

-- ==========================================================
-- POST MEDIA 
-- ==========================================================

-- Media for User 1 posts
INSERT INTO post_media (post_id, media_url, media_type, mime_type, sort_order, cid, ctime, mid, mtime)
VALUES
    (2001, 'https://images.unsplash.com/photo-1538428494232-9c0d8a3ab403?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1001, now(), 1001, now()),
    (2001, 'https://images.unsplash.com/photo-1545569341-9eb8b30979d9?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1001, now(), 1001, now()),
    (2001, 'https://images.unsplash.com/photo-1542051841857-5f90071e7989?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 3, 1001, now(), 1001, now()),

    (2002, 'https://images.unsplash.com/photo-1551632811-561732d1e306?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1001, now(), 1001, now()),
    (2002, 'https://images.unsplash.com/photo-1501554728187-ce583db33af7?q=80&w=387&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1001, now(), 1001, now()),

    (2003, 'https://images.unsplash.com/photo-1516483638261-f4dbaf036963?q=80&w=386&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1001, now(), 1001, now()),
    (2003, 'https://images.unsplash.com/photo-1523906834658-6e24ef2386f9?q=80&w=383&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1001, now(), 1001, now());

-- Media for User 2 posts
INSERT INTO post_media (post_id, media_url, media_type, mime_type, sort_order, cid, ctime, mid, mtime)
VALUES
    (2005, 'https://plus.unsplash.com/premium_photo-1661964177687-57387c2cbd14?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1002, now(), 1002, now()),
    (2005, 'https://images.unsplash.com/photo-1528164344705-47542687000d?q=80&w=892&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1002, now(), 1002, now()),
    (2005, 'https://plus.unsplash.com/premium_photo-1690749740487-01bbb8e51e71?q=80&w=465&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'video', 'video/mp4', 3, 1002, now(), 1002, now()),

    (2006, 'https://images.unsplash.com/photo-1465146344425-f00d5f5c8f07?q=80&w=876&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1002, now(), 1002, now()),
    (2006, 'https://images.unsplash.com/photo-1490750967868-88aa4486c946?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1002, now(), 1002, now()),

    (2007, 'https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?q=80&w=944&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1002, now(), 1002, now()),

    (2008, 'https://images.unsplash.com/photo-1513326738677-b964603b136d?q=80&w=449&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1002, now(), 1002, now()),
    (2008, 'https://images.unsplash.com/photo-1547448415-e9f5b28e570d?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1002, now(), 1002, now()),

    (2009, 'https://plus.unsplash.com/premium_photo-1692895424097-a195cfa8a0c6?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8bW9uZ29saWF8ZW58MHx8MHx8fDA%3D', 'image', 'image/jpeg', 1, 1002, now(), 1002, now());

-- Media for User 3 posts
INSERT INTO post_media (post_id, media_url, media_type, mime_type, sort_order, cid, ctime, mid, mtime)
VALUES
    (2010, 'https://plus.unsplash.com/premium_photo-1661962435210-e6cdbb2cbeb4?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1003, now(), 1003, now()),
    (2010, 'https://images.unsplash.com/photo-1554072675-66db59dba46f?q=80&w=873&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1003, now(), 1003, now()),
    (2010, 'https://plus.unsplash.com/premium_photo-1719843507795-585f21debf7f?q=80&w=871&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 3, 1003, now(), 1003, now()),

    (2011, 'https://mapster-test-123.oss-cn-shanghai.aliyuncs.com/1112659-hd_720_720_25fps.mp4', 'video', 'video/mp4', 1, 1003, now(), 1003, now()),
    (2011, 'https://images.unsplash.com/photo-1503917988258-f87a78e3c995?q=80&w=387&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1003, now(), 1003, now()),

    (2012, 'https://plus.unsplash.com/premium_photo-1716138192476-f34e85ad43c2?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1003, now(), 1003, now()),

    (2013, 'https://images.unsplash.com/photo-1613395877344-13d4a8e0d49e?q=80&w=435&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 1, 1003, now(), 1003, now()),
    (2013, 'https://plus.unsplash.com/premium_photo-1661964149725-fbf14eabd38c?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D', 'image', 'image/jpeg', 2, 1003, now(), 1003, now()),
    (2013, 'https://mapster-test-123.oss-cn-shanghai.aliyuncs.com/6555288-hd_1920_1080_25fps.mp4', 'video', 'video/mp4', 3, 1003, now(), 1003, now());
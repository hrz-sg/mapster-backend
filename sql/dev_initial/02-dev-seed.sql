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
INSERT INTO post (
    id,
    user_id,
    title,
    description,
    is_published,
    thumbnail_url,
    media_count,
    has_video,
    like_count,
    comment_count,
    saved_count,
    cid,
    ctime,
    mid,
    mtime
)
VALUES
    (
        2001,
        1001,
        'My first trip to Shanghai',
        'My first trip to Shanghai felt like stepping into the future.
The skyline looked unreal with its glowing lights.
I wandered along the Bund and admired the stunning contrast of old and new architecture.
The food scene amazed me at every corner.
I tried soup dumplings for the first time and fell in love instantly.
The city felt incredibly alive no matter the hour.
I also explored quiet parks hidden among tall buildings.
Every street had its own story.
I met friendly locals who helped me navigate the city.
This trip made Shanghai one of my favorite destinations.

Beyond the famous sights, I wandered through smaller neighborhoods and residential streets.
The metro system was fast and easy to use, which made exploring very convenient.
On a rainy evening, the reflections of neon lights on wet pavement made the city feel even more cinematic.
I spent time watching the boats on the river and listening to the sounds of traffic and conversations around me.
In between the busy moments, I found small cafes where I could sit, rest, and simply observe daily life.
By the end of the trip, I felt a strong connection to the city and a desire to return for an even longer stay.',
        TRUE,
        'https://images.unsplash.com/photo-1538428494232-9c0d8a3ab403?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        3,
        TRUE,
        125000,
        3400,
        22000,
        0,
        now(),
        0,
        now()
    ),
    (
        2002,
        1001,
        'Morgenwanderung im frühen Licht',
        'Eine Morgenwanderung hat etwas Magisches an sich.
Die Luft fühlt sich frischer an als zu jeder anderen Tageszeit.
Der Himmel beginnt langsam zu leuchten, bevor die Sonne aufgeht.
Der Weg war still und friedlich, nur begleitet von Vogelstimmen.
Jeder Schritt ließ mich wacher und leichter fühlen.
Der Duft von Kiefern und feuchtem Gras erfüllte den Wald.
Ich genoss den weiten Blick über das Tal, als ich den Gipfel erreichte.
Es war ein Moment völliger Ruhe.
Die Welt wirkte für einen Augenblick langsamer.
Diese Wanderung hat meinen ganzen Tag positiv beeinflusst.

Nach der Wanderung fühlte sich der restliche Tag besonders leicht an.
Ich nahm mir Zeit, tief durchzuatmen und den Moment bewusst wahrzunehmen.
Die Gedanken wurden klarer, je weiter ich ging.
Die ersten Sonnenstrahlen tauchten die Berge in ein warmes Licht.
Ab und zu blieb ich stehen, um die Stille zu genießen.
Am Ende des Weges war ich angenehm müde, aber innerlich voller neuer Energie und Dankbarkeit für diesen Morgen.',
        TRUE,
        'https://images.unsplash.com/photo-1551632811-561732d1e306?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        2,
        FALSE,
        3200,
        220,
        800,
        0,
        now(),
        0,
        now()
    ),
    (
        2003,
        1001,
        '美丽的日本旅程时光',
        '日本真的美得令人惊叹，它的每一个角落都像是精心描绘的画卷。
街道整洁而安静，让人一走进去便感到心情平静而舒畅。
清晨的空气带着淡淡的花香，仿佛在温柔地迎接所有来访的人。
寺庙的氛围非常宁静，让我忘记了时间，仿佛暂时脱离了日常生活的喧嚣。
漫步其中时，只有微风拂过树叶的声音，让人沉浸在完全的安宁中。

樱花在风中轻轻摇曳，像电影画面一样浪漫，每一片花瓣落下时都让人心动不已。
我在樱花树下停留了很久，只为多感受几分钟这种如梦似幻的景色。
这里的每一顿饭都精致又美味，从街边小吃到高档餐厅，都能让味蕾得到满足。
日本人对食物的用心和讲究让人佩服，每一道料理都像是一件艺术品。

传统文化和现代科技在这里完美融合，仿佛两个世界在同一空间中自然共存。
我走过的小巷都充满了细节与故事，古老的木屋、温暖的灯光、装饰细腻的店铺，让人忍不住放慢脚步。
商店里的人们非常礼貌且友好，每一次问候都带着真诚和温度，让旅途变得更加轻松愉快。

风景从城市到乡村都令人难忘。
城市里充满活力和秩序，而乡村的自然风光则纯粹而宁静。
无论是海边、山间还是田野，都有一种令人心安的美感。
夜晚的街道在灯光下闪闪发亮，夜市的热闹与白天的安静形成鲜明对比，让我体验到不同面貌的日本。

这次旅行让我更想再次回到日本，因为这里的美不仅在眼前，更深深留在心里。
每一个瞬间、每一个场景、每一次微小的互动都让我感受到日本独特的魅力。
离开时，我已经开始期待下一次更长、更深入的旅程。',
        FALSE,
        NULL,
        1,
        FALSE,
        980000,
        72000,
        410000,
        0,
        now(),
        0,
        now()
    );

-- Posts by User 2
INSERT INTO post (
    id,
    user_id,
    title,
    description,
    is_published,
    thumbnail_url,
    media_count,
    has_video,
    like_count,
    comment_count,
    saved_count,
    cid,
    ctime,
    mid,
    mtime
)
VALUES
    (
        2005,
        1002,
        'Удивительная и вдохновляющая Япония',
        'Япония снова удивила меня своей уникальностью, словно каждый раз открывает совершенно новую грань своей культуры и характера.
Каждый город здесь будто живёт по своим собственным правилам, создавая неповторимую атмосферу, в которой гармонично сочетаются традиции и современность.
Улицы чистые, аккуратные и невероятно спокойные, и даже в больших мегаполисах чувствуется особый порядок, который успокаивает и вдохновляет.

Я попробовал множество блюд, и каждое из них было произведением искусства — не только по вкусу, но и по подаче.
Даже простые блюда, которые можно найти в обычных семейных кафе, поражали вниманием к деталям.
В каждом вкусе ощущалась история, культура и философия.
Это был настоящий кулинарный опыт, который хочется пережить снова и снова.

Храмы и сады наполняют пространство тишиной и гармонией, создавая ощущение, будто время замедляется.
Там особенно легко почувствовать внутреннее равновесие.
Стоит лишь немного задержаться в таких местах, чтобы полностью отключиться от суеты и погрузиться в спокойствие.
Вечера же сияют неоновыми огнями, создавая совершенно другой мир — яркий, динамичный, живой.
Контраст между дневной тишиной и ночным сиянием делает Японию ещё более уникальной.

Я встречал людей, которые всегда готовы помочь, объяснить, подсказать дорогу или просто улыбнуться.
Их уважение, вежливость и искренность оставляют приятное послевкусие после общения.
Поезда ходят идеально точно — до секунд — что поражает каждый раз и делает путешествия невероятно удобными.

Фотографии не передают всю красоту этой страны: её атмосферу, настроение, запахи, звуки и маленькие детали, благодаря которым складывается целостное впечатление.
Эта поездка ещё сильнее укрепила мою любовь к Японии, сделав её одним из тех мест, куда всегда хочется возвращаться, чтобы открыть для себя что-то новое и снова почувствовать эту удивительную гармонию.',
        TRUE,
        'https://plus.unsplash.com/premium_photo-1661964177687-57387c2cbd14?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        3,
        TRUE,
        265000,
        18500,
        96000,
        0,
        now(),
        0,
        now()
    ),
    (
        2006,
        1002,
        'My Perfectly Peaceful Day Off',
        'My day off was exactly what I needed.
I started the morning slowly with a warm cup of coffee.
The quiet atmosphere made the day feel peaceful from the beginning.
I took a long walk and listened to my favorite music.
The weather was gentle and relaxing.
I spent time reading a book I had postponed for weeks.
Later, I cooked a simple but comforting meal.
I allowed myself to rest without feeling guilty.
The day felt balanced and refreshing.
Sometimes a calm day is the best gift you can give yourself.

Später am Nachmittag machte ich mir eine Playlist mit sanfter Musik und legte mich für eine Weile hin.
I looked out the window and watched the light slowly change as the day moved toward evening.
I wrote a few thoughts in a notebook to capture how calm and content I felt.
I also took a short break from my phone and social media to enjoy the quiet.
In the evening, I lit a small candle and enjoyed a warm drink before bed.
It was a simple day, but it reminded me that slowing down can be very powerful.
I went to sleep feeling rested, grounded, and ready for the days ahead.',
        TRUE,
        'https://images.unsplash.com/photo-1465146344425-f00d5f5c8f07?q=80&w=876&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        2,
        FALSE,
        870,
        45,
        210,
        0,
        now(),
        0,
        now()
    ),
    (
        2007,
        1002,
        'Urbanes Chicago: Energie der Großstadt',
        'Chicago ist eine Stadt voller Energie, die einen schon bei der Ankunft mit ihrer besonderen Dynamik empfängt.
Die Wolkenkratzer ragen beeindruckend in den Himmel und wirken je nach Tageszeit völlig unterschiedlich – morgens klar und kraftvoll, abends wie riesige leuchtende Skulpturen.
Die Straßen sind lebendig und voller Bewegung, und das ständige Summen der Stadt erzeugt eine Atmosphäre, die gleichzeitig aufregend und inspirierend ist.

An jeder Ecke entdeckt man Kunst und Kultur, sei es in Form von Straßenkunst, kleinen Galerien oder imposanten Museen, die Geschichten vergangener Jahrzehnte erzählen.
Der Wind vom See fühlt sich kühl und erfrischend an, besonders wenn man entlang des Lake Michigan spaziert.
Dort mischen sich das Rauschen des Wassers und das Treiben der Stadt auf eine Weise, die Chicago einzigartig macht.

Ich spazierte durch verschiedene Viertel, jedes mit seinem eigenen Charakter und seiner eigenen Stimmung.
Manche Straßen wirken historisch und ruhig, andere bunt und modern, voller kleiner Lokale, Boutiquen und Cafés.
Die Architektur ist vielfältig und eindrucksvoll: von ikonischen Hochhäusern bis hin zu detailreichen älteren Gebäuden, die an die industriellen Wurzeln der Stadt erinnern.

Das Essen in der Stadt ist ein echtes Highlight, denn die kulinarische Vielfalt Chicagos spiegelt die internationale Atmosphäre wider.
Von Deep-Dish-Pizza über Street Food bis hin zu gehobenen Restaurants findet man hier alles, was das Herz begehrt.
Menschen aus aller Welt prägen die Atmosphäre, wodurch die Stadt offen, warm und kulturell reich wirkt.

Chicago hat einen urbanen Charme, der lange im Gedächtnis bleibt.
Die Kombination aus moderner Energie, kultureller Tiefe und natürlicher Schönheit macht jeden Besuch zu einem besonderen Erlebnis.
Es ist eine Stadt, die sich ständig weiterentwickelt und dennoch ihre Seele bewahrt – und genau das macht sie so unvergesslich.',
        TRUE,
        'https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?q=80&w=944&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        1,
        FALSE,
        6400,
        390,
        1500,
        0,
        now(),
        0,
        now()
    ),
    (
        2008,
        1002,
        'Моё незабываемое путешествие в Москву',
        'Поездка в Москву получилась насыщенной и яркой.
Красная площадь впечатлила своими масштабами.
Архитектура здесь сочетает историю и современность.
Я гулял по центральным улицам и чувствовал атмосферу большого города.
Метро поразило своей красотой и величием.
На каждом шагу — музеи, культура и жизнь.
Вечером город светится мягкими огнями.
В парках ощущается спокойствие и уют.
Я попробовал традиционные блюда и остался в восторге.
Москва оставила сильное впечатление и желание вернуться.

Я зашёл в небольшие кофеенки и магазины, которые встретились по дороге, и в каждом месте чувствовался свой характер.
Особенно запомнились разговоры с местными жителями, которые с интересом рассказывали о городе и его истории.
Я сделал много фотографий, но вживую всё выглядело ещё масштабнее и ярче.
Один вечер я провёл, просто гуляя вдоль набережной и наблюдая, как огни города отражаются в воде.
Было чувство, что Москва — это город, в котором всегда есть чем заняться и что открыть для себя.
Эта поездка стала для меня не только туристическим опытом, но и личным воспоминанием о силе и красоте большого города.',
        TRUE,
        'https://images.unsplash.com/photo-1513326738677-b964603b136d?q=80&w=449&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        2,
        FALSE,
        45200,
        3100,
        9800,
        0,
        now(),
        0,
        now()
    ),
    (
        2009,
        1002,
        'Mongolian Horses and Endless Landscapes',
        'Seeing horses in Mongolia was an unforgettable experience.
They roam freely across vast open fields.
The landscape feels untouched and endless.
I watched local herders ride with incredible skill.
The connection between people and animals is very strong here.
The sound of hooves echoed across the plains.
The fresh wind carried the scent of grass and wilderness.
I felt a deep sense of peace in this open space.
The horses looked powerful yet gentle.
Mongolia’s nature and culture left a lasting mark on me.

In the evening, the sky over the steppe turned soft shades of orange and pink.
A quiet calm settled over the land as the day ended.
I listened to stories from the herders about their traditions and way of life.
Their deep respect for nature and animals was easy to feel in every word.
I realized how rare it is to experience such open space and silence.
This journey changed the way I think about freedom, distance, and connection to the land.',
        FALSE,
        NULL,
        1,
        FALSE,
        2100,
        160,
        630,
        0,
        now(),
        0,
        now()
    );

-- Posts by User 3
INSERT INTO post (
    id,
    user_id,
    title,
    description,
    is_published,
    thumbnail_url,
    media_count,
    has_video,
    like_count,
    comment_count,
    saved_count,
    cid,
    ctime,
    mid,
    mtime
)
VALUES
    (
        2010,
        1003,
        'Mein erholsamer Urlaub in Deutschland',
        'Mein Urlaub in Deutschland war voller schöner Eindrücke.
Die Städte waren sauber und gut organisiert.
Ich besuchte historische Schlösser, die beeindruckend erhalten waren.
Die Natur überraschte mich mit Wäldern und Seen.
Das Essen war abwechslungsreich und lecker.
Besonders gefiel mir die Ruhe in den kleinen Dörfern.
Die Menschen waren höflich und hilfsbereit.
Ich konnte viele neue Orte entdecken.
Die Architektur war elegant und voller Geschichte.
Dieser Urlaub war erfrischend und inspirierend.

Besonders beeindruckend waren die kleinen Details, die man erst auf den zweiten Blick bemerkt.
In den Altstädten fand ich enge Gassen mit Kopfsteinpflaster, in denen die Zeit langsamer zu vergehen schien.
An den Seen konnte ich lange Spaziergänge machen und den Blick über das ruhige Wasser schweifen lassen.
In den Cafés herrschte eine gemütliche Atmosphäre, in der man leicht ins Gespräch mit anderen Gästen kam.
Jede Region hatte ihre eigenen Traditionen und Spezialitäten, die ich nach und nach entdecken durfte.
Diese Vielfalt machte die Reise durch Deutschland noch spannender und bereichernder.',
        TRUE,
        'https://plus.unsplash.com/premium_photo-1661962435210-e6cdbb2cbeb4?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        3,
        FALSE,
        12800,
        870,
        4200,
        0,
        now(),
        0,
        now()
    ),
    (
        2011,
        1003,
        'Business trip to France',
        'My business trip to France was both productive and enjoyable.
The meetings went smoothly and gave me new ideas.
I walked through charming streets during my free time.
The cafés were perfect for quick breaks and inspiration.
The architecture made every walk feel special.
I enjoyed simple yet delicious meals.
The people I met were welcoming and professional.
The trip helped me see the country from a different perspective.
I felt motivated and refreshed.
France proved that even work trips can feel memorable.

In the evenings, I often walked back to the hotel through softly lit streets.
The city felt different at night, calmer but still full of quiet movement.
I reflected on the conversations from the day and how they could shape future projects.
Between meetings, I managed to visit a few landmarks and enjoy short moments of sightseeing.
These small breaks helped me stay balanced and focused.
By the end of the trip, I felt not only professionally satisfied but also personally enriched by the experiences and impressions from France.',
        TRUE,
        'https://mapster-test-123.oss-cn-shanghai.aliyuncs.com/1112659-hd_720_720_25fps.mp4',
        2,
        TRUE,
        534000,
        36500,
        187000,
        0,
        now(),
        0,
        now()
    ),
    (
        2012,
        1003,
        'Испания — удивительное место для путешествий',
        'Испания оказалась ещё одним потрясающим местом для путешествий.
Улицы наполнены солнечным светом и яркими красками.
Я гулял по старым кварталам и наслаждался архитектурой.
Испанская еда была невероятно вкусной и ароматной.
Особенно понравились тапас и паэлья.
Музыка и танцы слышны почти везде.
Люди дружелюбные и открытые.
Пляжи чистые и уютные.
Атмосфера здесь расслабленная и радостная.
Испания легко покоряет сердце путешественника.

Я также нашёл время заглянуть в небольшие семейные заведения, где подают домашнюю еду и с радостью общаются с гостями.
Вечерами я просто гулял без цели, наслаждаясь огнями улиц и живой атмосферой.
Иногда я останавливался на площади, чтобы посмотреть на уличных музыкантов и танцоров.
Эти маленькие моменты создавали особое ощущение близости к городу.
Мне захотелось когда-нибудь вернуться сюда ещё раз и провести больше времени в разных регионах Испании.
Эта поездка оставила в памяти тёплые и радостные воспоминания.',
        TRUE,
        'https://plus.unsplash.com/premium_photo-1716138192476-f34e85ad43c2?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        1,
        FALSE,
        5900,
        280,
        1700,
        0,
        now(),
        0,
        now()
    ),
    (
        2013,
        1003,
        '令人惊叹的希腊海岛之旅',
        '希腊真的让我惊叹不已。
蓝色的大海让人目不转睛。
白色的小房子在阳光下闪耀。
岛上的空气带着咸味和自由的气息。
食物非常新鲜又美味。
古老的遗迹让人仿佛穿越时空。
每个日落都像一幅画。
当地人热情又亲切。
海风吹来时让人彻底放松。
希腊真的是一个让人想再去一次的地方。

我还花时间在小巷里漫步，发现一些不那么热门但非常迷人的角落。
当地的咖啡馆和小店布置得很有特色，让人忍不住多看几眼。
有人在街头弹吉他，也有人悠闲地坐在露天餐桌前聊天。
夜晚的岛屿同样迷人，灯光映在海面上，微波荡漾，仿佛在轻声诉说故事。
我在这里拍了很多照片，但最难忘的还是当下的感受和心情。
离开时，我带走的不只是纪念品，还有对希腊海岛生活方式的向往和回忆。',
        TRUE,
        'https://images.unsplash.com/photo-1613395877344-13d4a8e0d49e?q=80&w=435&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        3,
        TRUE,
        1200000,
        95500,
        520000,
        0,
        now(),
        0,
        now()
    );

-- ==========================================================
-- POST MEDIA
-- ==========================================================

-- Media for User 1 posts
INSERT INTO post_media (
    post_id,
    media_url,
    media_type,
    mime_type,
    sort_order,
    cid,
    ctime,
    mid,
    mtime
)
VALUES
    (
        2001,
        'https://images.unsplash.com/photo-1538428494232-9c0d8a3ab403?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1001,
        now(),
        1001,
        now()
    ),
    (
        2001,
        'https://plus.unsplash.com/premium_photo-1729162773996-68e1c42d77a8?q=80&w=774&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1001,
        now(),
        1001,
        now()
    ),
    (
        2001,
        'https://images.unsplash.com/photo-1474181487882-5abf3f0ba6c2?q=80&w=1740&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        3,
        1001,
        now(),
        1001,
        now()
    ),
    (
        2002,
        'https://images.unsplash.com/photo-1551632811-561732d1e306?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1001,
        now(),
        1001,
        now()
    ),
    (
        2002,
        'https://plus.unsplash.com/premium_photo-1677002240252-af3f88114efc?q=80&w=1650&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1001,
        now(),
        1001,
        now()
    ),
    (
        2003,
        'https://images.unsplash.com/photo-1478436127897-769e1b3f0f36?q=80&w=1740&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1001,
        now(),
        1001,
        now()
    ),
    (
        2003,
        'https://images.unsplash.com/photo-1480796927426-f609979314bd?q=80&w=1740&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1001,
        now(),
        1001,
        now()
    );

-- Media for User 2 posts
INSERT INTO post_media (
    post_id,
    media_url,
    media_type,
    mime_type,
    sort_order,
    cid,
    ctime,
    mid,
    mtime
)
VALUES
    (
        2005,
        'https://plus.unsplash.com/premium_photo-1661964177687-57387c2cbd14?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2005,
        'https://images.unsplash.com/photo-1528164344705-47542687000d?q=80&w=892&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2005,
        'https://plus.unsplash.com/premium_photo-1690749740487-01bbb8e51e71?q=80&w=465&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'video',
        'video/mp4',
        3,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2006,
        'https://images.unsplash.com/photo-1465146344425-f00d5f5c8f07?q=80&w=876&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2006,
        'https://images.unsplash.com/photo-1490750967868-88aa4486c946?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2007,
        'https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?q=80&w=944&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2008,
        'https://images.unsplash.com/photo-1513326738677-b964603b136d?q=80&w=449&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2008,
        'https://images.unsplash.com/photo-1547448415-e9f5b28e570d?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1002,
        now(),
        1002,
        now()
    ),
    (
        2009,
        'https://plus.unsplash.com/premium_photo-1692895424097-a195cfa8a0c6?w=600&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8bW9uZ29saWF8ZW58MHx8MHx8fDA%3D',
        'image',
        'image/jpeg',
        1,
        1002,
        now(),
        1002,
        now()
    );

-- Media for User 3 posts
INSERT INTO post_media (
    post_id,
    media_url,
    media_type,
    mime_type,
    sort_order,
    cid,
    ctime,
    mid,
    mtime
)
VALUES
    (
        2010,
        'https://plus.unsplash.com/premium_photo-1661962435210-e6cdbb2cbeb4?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2010,
        'https://images.unsplash.com/photo-1554072675-66db59dba46f?q=80&w=873&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2010,
        'https://plus.unsplash.com/premium_photo-1719843507795-585f21debf7f?q=80&w=871&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        3,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2011,
        'https://mapster-test-123.oss-cn-shanghai.aliyuncs.com/1112659-hd_720_720_25fps.mp4',
        'video',
        'video/mp4',
        1,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2011,
        'https://images.unsplash.com/photo-1503917988258-f87a78e3c995?q=80&w=387&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2012,
        'https://plus.unsplash.com/premium_photo-1716138192476-f34e85ad43c2?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2013,
        'https://images.unsplash.com/photo-1613395877344-13d4a8e0d49e?q=80&w=435&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        1,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2013,
        'https://plus.unsplash.com/premium_photo-1661964149725-fbf14eabd38c?q=80&w=870&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D',
        'image',
        'image/jpeg',
        2,
        1003,
        now(),
        1003,
        now()
    ),
    (
        2013,
        'https://mapster-test-123.oss-cn-shanghai.aliyuncs.com/6555288-hd_1920_1080_25fps.mp4',
        'video',
        'video/mp4',
        3,
        1003,
        now(),
        1003,
        now()
    );

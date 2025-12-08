INSERT INTO
  manga (
    manga_id,
    title,
    short_title,
    crawl_url,
    public_url,
    episode,
    portal_kind,
    created_at,
    updated_at
  )
VALUES
  (
    '11111111-1111-1111-1111-111111111111',
    'ワンパンマン',
    'one_punch_man',
    'https://tonarinoyj.jp/rss/series/13932016480028984490',
    'https://tonarinoyj.jp/episode/13932016480028985383',
    'Episode 1',
    'TonarinoYJ',
    NOW(),
    NOW()
  ),
  (
    '22222222-2222-2222-2222-222222222222',
    '盾の勇者の成り上がり',
    'shield_hero',
    'https://comic-walker.com/detail/KC_001105_S?episodeType=latest',
    'https://comic-walker.com/detail/KC_001105_S?episodeType=latest',
    '第1話',
    'KadoComi',
    NOW(),
    NOW()
  ) ON CONFLICT DO NOTHING;

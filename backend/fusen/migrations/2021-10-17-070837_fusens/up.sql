CREATE TABLE fusens (
    id VARCHAR(64) PRIMARY KEY,
    title VARCHAR(64) NOT NULL,
    note TEXT NOT NULL,
    create_at timestamptz NOT NULL DEFAULT current_timestamp,
    update_at timestamptz NOT NULL DEFAULT current_timestamp
);
INSERT INTO fusens (id, title, note)
VALUES (
        '0123456789ABCDEFGHJKMNPQRSTVWXYZ',
        'daprでつくるマイクロサービス',
        '## はじめに\n\nこの記事は、 [富士通クラウドテクノロジーズ Advent Calendar 2020](https://qiita.com/advent-calendar/2020/fjct) の**2日目**の記事です。\n\n1日目は @miyuush さんの **[ニフクラがTerraformに対応したので使ってみた【基礎編】](https://blog.pfs.nifcloud.com/20201201_terraform_provider_nifcloud)** でした！'
    );

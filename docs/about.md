---
marp: true
theme: gaia
---
# rust-axum-async-graphal-blog

Rust/axum/async-graphql を使用したブログシステム

---
# 目次

1. 要件と挙動の確認
2. システムの構成
3. 各コンポーネントの詳細
4. 今後の課題

---
# 要件と挙動の確認

1. [構成] Rust/axum/async-graphql を使用した Blog システム
2. [実行環境] 何らかの動作を確認する方法が提供されていること
    - docker-compose で実行が可能
3. [機能] 当該 Blog システムは以下の機能を備えること
    - 一覧取得/投稿/詳細取得/更新/削除
4. [開発環境] git を使用して開発すること
    - github にて開発 → 確認は省略
    - [kaznak/202203.rust-axum-async-graphql-blog](https://github.com/kaznak/202203.rust-axum-async-graphql-blog)

---
# 要件と挙動の確認: [構成]

Rust/axum/async-graphql を使用

`backend/Cargo.toml`

```
async-graphql = "3.0.36"        # ←
async-graphql-axum = "3.0.36"
axum = "0.4"                    # ←
tokio = { version = "1.8", features = ["macros", "rt-multi-thread"] }
hyper = "0.14"
tower-http = { version = "0.2.1", features = ["cors"] }
futures = "0.3.0"
```

---
# 要件と挙動の確認: [構成]

- バックエンド
    - Rust/axum/async-graphql
    - 投稿データは Markdown でファイルシステムに格納
        - メタデータは frontmatter に格納
- フロントエンド
    - next.js/tailwindcss/react-markdown
    - swr/graphql-request

詳細な構成は各コンポーネントの詳細にて確認

---
# 要件と挙動の確認: [実行環境]

docker-compose で実行

実際に実行します。

```
$ docker-compose up
Starting 202203rust-axum-async-graphal-blog_frontend_1 ... done
Starting 202203rust-axum-async-graphal-blog_backend_1  ... done
Attaching to 202203rust-axum-async-graphal-blog_frontend_1, 202203rust-axum-async-graphal-blog_backend_1
backend_1   | Playground: http://localhost:8000
frontend_1  | Listening on port 3000
```

この環境で機能を確認します

---
# 要件と挙動の確認: [機能] 一覧取得

[http://localhost:3000/posts]()

![](img/screen-list.png)

---
# 要件と挙動の確認: [機能] 投稿

[http://localhost:3000/posts-editor]()

![](img/screen-create.png)

---
# 要件と挙動の確認: [機能] 詳細取得

[http://localhost:3000/posts/slkajgoaijdgova]()

![](img/screen-post.png)

---
# 要件と挙動の確認: [機能] 更新

[http://localhost:3000/posts-editor/slkajgoaijdgova]()

![](img/screen-update.png)

---
# 要件と挙動の確認: [機能] 削除

[http://localhost:3000/posts-editor/slkajgoaijdgova]()

![](img/screen-update.png)

---
# 各コンポーネントの詳細
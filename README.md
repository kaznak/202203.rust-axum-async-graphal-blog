# 202203.rust-axum-async-graphql-blog

## Requirements

- [X] Rust/axum/async-graphql を使用した Blog システム
    - 当該ライブラリ使用
- [X] git を使用して開発すること
    - github にて開発
- [X] 何らかの動作を確認する方法が提供されていること
    - docker-compose で実行が可能
- [ ] 当該 Blog システムは以下の機能を備えること

| | バックエンド | BFF | フロントエンド |
| -- | -- | -- | -- |
| 投稿 | ✔️ | ✔️ | ✔️ |
| 一覧取得 | ✔️ | ✔️ | ✔️ |
| 詳細取得 | ✔️ | ✔️ | ✔️ |
| 更新 | ✔️ | ✔️ | ✔️ |
| 削除 | ✔️ | ✔️ | ✔️ |

## Reference

- [Rust を学ぶ](https://www.rust-lang.org/ja/learn)
- [axum](https://github.com/tokio-rs/axum)
- [async-graphql](https://github.com/async-graphql/async-graphql)
    - [examples](https://github.com/async-graphql/examples)
    - [document](https://async-graphql.github.io/async-graphql/en/index.html)
- [nextjs]()
    - [examples/with-docker](https://github.com/vercel/next.js/tree/canary/examples/with-docker)
- [docker compose](https://docs.docker.com/compose/)

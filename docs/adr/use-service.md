# src/service にビジネスロジックを切り出す

## Context

main.rs にすべてを書くのは悪手なので、どうにか切り出したい。

## Decision

- src/service/xx.rs として、ビジネスロジックをセクションごとに切り出す。
- src/service/utils.rs に、各セクションを横断して利用する関数を記述する
- すべての src/service/xx.rs は、src/service.rs からモジュールを公開する
  - 公開するモジュールは main.rs で利用する必要最低限の関数のみとする

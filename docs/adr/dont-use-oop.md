# オブジェクト指向っぽいプログラミング手法を使わない

## Context

DIP を用いた DI をする上で、どのように依存性を注入するかで迷った。
Service.rs や xxx_service.rs で構造体を定義し、コンストラクタで infra をもらうことを考えた。
が、複雑で何をしているか把握できるような設計に落とし込むことができないと判断し、利用しないこととした。
内部に状態を持つことを許容した構造になってしまうこと、各関数が完全に分離しないことが複雑さを煩く引き上げてしまうと判断した。

## Decision

- 層を wrap する意図を持った構造体は採用しない
- 記述量は増えるが、各関数の引数に infra を定義する

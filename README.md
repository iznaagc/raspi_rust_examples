# Rust + Raspberry Pi GPIO Examples

## ブランチ説明

main: 動作確認完了済みのもの
development: 開発途中のもの

## 概要

ラズパイでGPIOを制御するサンプルコード群です。

このリポジトリはRust版なので
Python版やGo版もあるのでお好みでご利用ください。

## 注意点

PythonやGoとは違いコンパイルが必要なため、
ディレクトリなどの命名がRust向けに若干変化しています。

後はRustの特性上src/main.rsがエントリーポイントになるため、
使用する際はモジュール名を変更して呼び出す関数を変えてやる必要があるので注意です。

デフォルトではLチカのサンプルが動作するようになっています。

## Lチカ(led_flash_001)

電子工作版「Hello World」が動作。

GPIO 17番ピン(BCM)に接続したLEDが1秒間隔で点滅します。



このリポジトリは[実践Rustプログラミング入門](https://www.shuwasystem.co.jp/book/9784798061702.html) 10章の内容を写経したものです。

`wordcount` はシンプルな文字、単語、行の出現頻度の計数機能を提供します。
CLIから単語数の出現頻度を使うことができます。

```console
$ cargo run text.txt
{"cc": 1, "aa": 2, "bb": 1}
```
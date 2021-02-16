# Sudoku Solver
## 数独の形式
9x9 のグリッドで、空欄は `?` で置いてください。サンプルは `sample/` 下にあります。

## 実行
リリースビルドで実行してください。Rust の High Performance を experience するためです。
```console
$ cargo run --release
```
グリッドを入力するのがめんどくさいならこちらを
```console
$ cargo run --release < sample/most-difficult.txt
```
# 深さ優先探索と幅優先探索の比較
見てもらった通り、そんなに変わらない

## 幅優先探索でも遅い理由
「冗長な探索」の排除をしていないから

## 「冗長な探索」
探索中の命令（コード中では`Instruction`）に対して、
- その命令は入力された正規表現から生成された命令の列（コード中では`inst: &[Instruction]`）の先頭から何番目の命令か（コード中では`pc`）
- 現在入力文字列（コード中では`line`）の何文字目を見ているか（コード中では`sp`）
が一致しているとき、「冗長な探索」（後の探索結果が必ず一致する）

## 参考
[Thompson vmの解説](https://swtch.com/~rsc/regexp/regexp2.html#:~:text=Thompson%27s%20Implementation)

[Thompson vmのコード(Thompson.c)](https://code.google.com/archive/p/re1/)
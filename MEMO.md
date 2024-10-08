# cargo
`cargo new` ... プロジェクト作成  
`cargo build` ... ビルド  
`cargo build --release` ... 最適化されたビルド  
`cargo run` ... ビルドと実行をいっぺんにやる
`cargo check` ... コンパイル可能かチェックするだけ  

# 文と式
文 ... 何らかの動作をして、値を返さない。  
式 ... 結果として値に評価される。式はセミコロンで終わらない（セミコロンがつくと、文になる）。  

# 関数の戻り値
Rustでは、関数の戻り値は、関数本体ブロックの最後の式の値と同じ。

# 所有権（ownership）
あるデータに対して、そのメモリを所有している変数がどれか、という話  

- Rustの各値は、所有者と呼ばれる変数と対応している。
- いかなる時も所有者は一つである。
    - （Rust）メモリの二重解放が起こらないようにしている
- 所有者がスコープから外れたら、値は破棄される。

# 借用（borrowing）
関数の引数に参照を取ること。  

# 参照の制約
- 不変な参照をしている間は、可変な参照はできない
    - 不変な参照は複数同時にできる
- 可変な参照をしている間は、不変な参照はできない
    - 可変な参照は同時に1つだけ
    - （Rust）データの競合が起きないようになっている
- 参照は常に有効でないといけない
    - (Rust) ダングリングポインタが出来ないような仕組みになっている

# スマートポインタ
- ポインタのように振る舞うだけでなく、追加のメタデータと能力があるデータ構造
- 参照はデータを借用するだけのポインタ。対照的に多くの場合、スマートポインタは指しているデータを所有する。
- DerefとDropトレイトを実装している
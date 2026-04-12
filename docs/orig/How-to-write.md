@note
- Edit Time: 2026-04-08 13:30 First Edit
@end
@jp
## 書き方
本項は、図形を描する上での制約条件と、SPCにおける「描画の哲学」を説明します。

# 一行の欺瞞
単一の円や線を描くだけなら、SPCは極めて簡潔な記述で機能します。しかし、複数の図形を組み合わせ、意図した通りの「絵」を構築するためには、いくつかのルールを守る必要があります。

# 描画における制約
- **順次上書き**: ファイルの先頭行から最終行へと順次処理されます。常に、「後から書いたものが上に重なる」という性質を利用し、背後から手前へと描き進めてください。
- **操作タグの配置**: RC/GR/ROといった描画環境を定義するタグは、、一般タグの直後、図形定義の前に記述する必要があります。
- **曲線の限界**: ARタグによる曲線は、ドットの解像度に依存するため、完全な滑らかさを保証するものではありません。
- **回転の制限**: RO(回転)とTE(テキスト)の組み合わせは、SVG出力モード限定の機能です。

# タグの記述順序
記述ミスによる予期せぬ挙動を防ぐため、以下の順序を遵守してください。
1. **一般タグ** (VN/FN/FT/FH/FW/BG/LW/LC)
2. **操作タグ** (RC/GR/RO)
3. **矩形タグ** (LN/CI/EC/AR/RE/PG/TE)
4. **操作タグ** (PL/RP)
@end
@en
## How to Write
This section describes the constraints and the fundamental philosophy of drawing with SPC.

# The Deception of the One-Liner
Drawing a single circle or line is simple with just one line of code. However, when combining multiple shapes to build intended imagery, there are key rules to follow.

# Constraints on drawing
- **Sequential Overwriting**: Commands are processed in order from the first line to the last. This means newer objects always layer on top. Leverage this "painting over" logic to refine your shapes from background to foreground.
- **Placement of Operation Tags**: Control tags such as RC, GR, and RO should be declared immediately after Generic Tags and before any shape definitions.
- **Limitations of Curves**: AR (Arc) tags are limited by pixel resolution and do not produce perfectly smooth curves.
- **Rotation Constraints**: Rotating text (the RO + TE combination) is a feature exclusive to SVG output mode. 

# Tag Sequence
To avoid unexpected behavior, the following declaration order is recommended:
1. **Generic Tag** (VN/FN/FT/FH/FW/BG/LW/LC)
2. **Operation Tag** (RC/GR/RO)
3. **Shapes Tag** (Ln/CI/EC/AR/RE/PG/TE)
4. **Operation Tag** (PL/RP)
@end
#----------------------------------------------------------------------

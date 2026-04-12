@note
- Edit Time: 2026-02-14 18:00 First Edit
@end
@jp
## PGタグ
- 正多角形を描画するタグ
- 原点は中心。頂点までの距離は半径。
- 描画開始点は、X軸上。

@end
@en
## PG Tag
- Regular Polygon drawing Tag
- origin is center. length of vertex is radius.
- drawing start point is on x-axis.

@end
@jp
# 命令
|タグ名| 要素名|
@end
@en
# Ordering
|Tag| String|
@end
@common
|:-:|:-:|
|PG| x, y, r, n, angle [, stroke_color, width, fill_color] |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| PG           | PGタグ       | string |
| x            | 中心のX座標  | float32|
| y            | 中心のY座標  | float32|
| r            | 半径         | float32|
| n            | 頂点数       | float32|
| angle        | 回転角       | float32|
| stroke_color | 辺の色       | string |
| stroke_width | 辺の幅       | float32|
| fill_color   | 内部の色     | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| PG           | PG Tag                 | string |
| x            | X coord of center      | float32|
| y            | Y coord of center      | float32|
| r            | Radius                 | float32|
| n            | Vertex number          | int32  |
| stroke_color | Stroke color           | string |
| stroke_width | Stroke width           | float32|
| fill_color   | Internal color         | string |
@end

@jp
# 例題 (OKパターン)
@end
@en
# Example (OK Pattern)
@end
@common
```bash
#
PG,0,0,50,3,0.0,yellow,2.0,red
```
@end

@jp
# 例題 (NGパターン)
@end
@en
# Example (NG Pattern)
@end
@common
```bash
#
PG,0,0,50,6,90.0,yellow,fill
```
@end

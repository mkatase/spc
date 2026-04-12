@note
- Edit Time: 2026-02-14 15:30 First Edit
@end
@jp
## REタグ
- 矩形を描画するタグ

@end
@en
## RE Tag
- Rectangle drawing Tag

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
|RE| x, y, w, h [, stroke_color, width, fill_color] |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| RE           | REタグ       | string |
| x            | 左下のX座標  | float32|
| y            | 左下のY座標  | float32|
| w            | 幅（X方向）  | float32|
| h            | 高さ（Y方向）| float32|
| stroke_color | 辺の色       | string |
| stroke_width | 辺の幅       | float32|
| fill_color   | 内部の色     | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| RE           | RE Tag                 | string |
| x            | X coord of Bottom-Left | float32|
| y            | Y coord of Bottom-Left | float32|
| w            | Width                  | float32|
| h            | Height                 | float32|
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
RE,10,10,50,100,yellow,2.0,red
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
RE,10,10,50,yellow
```
@end

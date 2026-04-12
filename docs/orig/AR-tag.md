@note
- Edit Time: 2026-02-14 14:40 First Edit
@end
@jp
## AR タグ
- 円弧を描くタグ
@end
@en
## AR Tag
- Arc drawing Tag
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
|AR| cx, cy, rx, ry [, stroke_color, width, fill_color] |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| AR          | ARタグ                              | string |
| cx          | 中心のX座標                         | float32|
| cy          | 中心のY座標                         | float32|
| rx          | X方向(横)の半径                     | float32|
| ry          | Y方向(縦)半径                       | float32|
|storoke_color| 枠色指定（カラーマップ参照）省略可。| string |
|width        | 線幅（fill指定とセット)省略可。     | float32|
|fill_color   | 色指定（カラーマップ参照）省略可。  | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| AR         | AR Tag                                    | string |
| cx         | Center Point of X-Axis                    | float32|
| cy         | Center Point of Y-Axis                    | float32|
| rx         | Radius of X-Axis                          | float32|
| ry         | Radius of Y-Axis                          | float32|
|stroke_color| Color String(refer to Color Map) Optional | string |
|width       | Line Width(using with fill) Optional      | float32|
|fill_color  | Fill String(using with width) Optional    | string |
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
EC,0,0,10,20
EC,0,20,10,20,yellow,2.0
EC,40,40,10,20,red,2.0,blue
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
EC,0,0,100,0
EC,0,20,50,20,yellow,3.0,fill
```
@end

@note
- Edit Time: 2026-02-14 14:40 First Edit
@end
@jp
## CI タグ
- 円を描くタグ
@end
@en
## CI Tag
- Circle drawing Tag
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
|CI| cx, cy, r [, stroke_color, width, fill_color] |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| CI          | CIタグ                              | string |
| cx          | 中心のX座標                         | float32|
| cy          | 中心のY座標                         | float32|
|  r          | 半径                                | float32|
|storoke_color| 枠色指定（カラーマップ参照）省略可。| string |
|width        | 線幅（fill指定とセット)省略可。     | float32|
|fill_color   | 色指定（カラーマップ参照）省略可。  | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| CI         | CI Tag                                    | string |
| cx         | Center Point of X-Axis                    | float32|
| cy         | Center Point of Y-Axis                    | float32|
|  r         | Radius                                    | float32|
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
CI,0,0,10
CI,0,20,10,yellow,2.0
CI,40,40,10,red,2.0,blue
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
CI,0,0,100,0
CI,0,20,50,20,yellow,3.0
```
@end

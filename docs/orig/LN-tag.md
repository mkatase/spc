@note
- Edit Time: 2026-02-14 11:20 First Edit
@end
@jp
## LN タグ
- 実線を描くタグ
@end
@en
## LN Tag
- Solid line drawing Tag
@end

@jp
# 命令
|タグ名| 要素名|
@end
@en
# Ordering
|Tag| String|
@end
@comm
|:-:|:-:|
|LN| x1, y1, x2, y2 [, color, width] |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| LN  | LNタグ                            | string |
| x1  | 始点のX座標                       | float32|
| y1  | 始点のY座標                       | float32|
| x2  | 終点のX座標                       | float32|
| y2  | 終点のY座標                       | float32|
|color| 色指定（カラーマップ参照）省略可。| string |
|width| 線幅（fill指定とセット)省略可。   | float32|
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| LN  | LN Tag                                    | string |
| x1  | Start Point of X-Axis                     | float32|
| y1  | Start Point of Y-Axis                     | float32|
| x2  | End Point of X-Axis                       | float32|
| y2  | End Point of Y-Axis                       | float32|
|color| Color String(refer to Color Map) Optional | string |
|width| Line Width(using with fill) Optional      | float32|
|fill| Fill String(using with width) Optional     | string |
@end

@jp
# 例題 (OKパターン)
@end
@en
# Example (OK Pattern)
@end
@comm
```bash
#
LN,0,0,100,0
LN,0,20,50,20,yellow
```
@end

@jp
# 例題 (NGパターン)
@end
@en
# Example (NG Pattern)
@end
@comm
```bash
#
LM,0,0,100,0
LN,0,20,50,20,yellow,3.0
```
@end

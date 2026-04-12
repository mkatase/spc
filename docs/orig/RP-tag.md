@note
- Edit Time: 2026-02-16 23:00 First Edit
@end
@jp
## RPタグ
- 指定したマーカーを複数配置するタグ
- GR/ROタグとセットで使用する

@end
@en
## RP Tag
- Target marker tag multi-placement tag
- Combined GR/RO tag

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
|RP| marker, count_x, count_y, base_x, base_y, step_x, step_y |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| RP      | RPタグ                  | string |
| marker  | 対象マーカー            | string |
| count_x | X方向の繰り返し回数     | int32  |
| count_y | Y方向の繰り返し回数     | int32  |
| base_x  | X方向の繰り返し起点座標 | float32|
| base_y  | Y方向の繰り返し起点座標 | float32|
| step_x  | X方向の繰り返し間隔回数 | float32|
| step_y  | Y方向の繰り返し間隔回数 | float32|
- 対象マーカーは＠マークから開始(@xxx)
- xxx部に文字数制限はないが、簡潔がよい
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| RP      | RP Tag                      | string |
| marker  | Target marker               | string |
| count_x | repeat count of X-direction | int32  |
| count_y | repeat count of Y-direction | int32  |
| base_x  | X coord of start point      | float32|
| base_y  | Y coord of start point      | float32|
| step_x  | Spacing in X-direction      | float32|
| step_y  | Spacing in X-direction      | float32|
- 1st char of Target marker is '@' (@xxx)
- part of 'xxx' is easy string
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
RP,@G1,5,5,10,10,20,20
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
RP,P1,5,5,10,10
```
@end

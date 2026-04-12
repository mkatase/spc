@note
- Edit Time: 2026-02-16 22:20 First Edit
@end
@jp
## PLタグ
- 指定したマーカーを配置するタグ
- GR/ROタグとセットで使用する

@end
@en
## PL Tag
- Target marker tag placement tag
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
|PL| marker, x, y |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| PL    | PLタグ         | string |
| marker| 対象マーカー   | string |
| x     | 開始のX座標    | float32|
| y     | 開始のY座標    | float32|
- 対象マーカーは＠マークから開始(@xxx)
- xxx部に文字数制限はないが、簡潔がよい
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| PL    | PL Tag                 | string |
| marker| Target marker          | string |
| x     | X coord of start point | float32|
| y     | Y coord of start point | float32|
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
PL,@G1,20,30
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
PL,P1,20,20,0
```
@end

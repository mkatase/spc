@note
- Edit Time: 2026-02-14 11:20 First Edit
@end
@jp
## LWタグ
- 全体的な線幅を示すタグ
- 省略可能。デフォルトは、"1.0"
- 個別タグにて、線幅は変更可能

@end
@en
## LW Tag
- Global line width Tag
- This tag is optional. Default is "1.0".
- Line width is changable by each tag.

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
|LW| width |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| LW    | LWタグ              | string  |
| width | 線幅                | float32 |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| LW    | LW Tag             | string  |
| width | Line width         | float32 |
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
LW,2.0
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
LW,3
```
@end

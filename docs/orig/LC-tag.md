@note
- Edit Time: 2026-02-14 14:20 First Edit
@end
@jp
## LCタグ
- 全体的な線色を示すタグ
- 省略可能。デフォルトは、"white"
- 個別タグにて、線幅は変更可能

@end
@en
## LW Tag
- Global line color Tag
- This tag is optional. Default is "white".
- Line color is changable by each tag.

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
|LC| color |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| LC    | LCタグ              | string |
| color | 線色                | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| LC    | LC Tag             | string |
| color | Line color         | string |
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
LC,orange
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
LC,3
```
@end

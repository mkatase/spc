@note
- Edit Time: 2026-02-14 14:35 First Edit
@end
@jp
## BGタグ
- 背景色を示すタグ
- 省略可能。デフォルトは、"black"
- 選択色は、カラーマップからのみ

@end
@en
## BG Tag
- Global background color Tag
- This tag is optional. Default is "black".
- Selectable color is in Color-Map List only.

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
|BG| color |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| BG    | BGタグ              | string |
| color | 背景色              | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| BG    | BG Tag             | string |
| color | Background Color   | string |
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
BG,dark_gray
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
BG,#00ff00
```
@end

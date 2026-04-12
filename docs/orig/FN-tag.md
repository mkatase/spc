@note
- Edit Time: 2026-02-14 11:00 First Edit
@end
@jp
## FNタグ
- 出力ファイル名を示すタグ
- 省略可能。デフォルトは、"spc"。

@end
@en
## FN Tag
- File name Tag
- This tag is optional. Default is "spc".

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
|FN| name |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| FN   | FNタグ              | string |
| name | ファイル名          | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| FN   | FN Tag             | string |
| name | File name          | string |
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
FN,sample
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
FN,\aaa   # meta charactor is dangerous.
```
@end

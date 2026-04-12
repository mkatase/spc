@note
- Edit Time: 2026-02-14 11:10 First Edit
@end
@jp
## FTタグ
- 出力ファイル名の拡張子を示すタグ
- 省略可能。デフォルトは、"webp"
- 利用可能な拡張子は、"webp"と"png"

@end
@en
## FT Tag
- File Extension Tag
- This tag is optional. Default is "webp".
- Available extension is "webp" and "png".

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
|FT| name |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| FT   | FTタグ              | string |
| name | 拡張子名            | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| FT   | FT Tag              | string |
| name | File extension name | string |
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
FT,webp
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
FT,jpg
```
@end

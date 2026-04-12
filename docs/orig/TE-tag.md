@note
- Edit Time: 2026-02-14 18:20 First Edit
@end
@jp
## TEタグ
- テキストを描画するタグ
- Noto Sans Fontを使用

@end
@en
## TE Tag
- Text drawing tag
- Adopted Noto Sans Font

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
|TE| x, y, text, [, color, size] |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| TE    | TEタグ         | string |
| x     | 開始のX座標    | float32|
| y     | 開始のY座標    | float32|
| text  | テキスト       | float32|
| color | テキスト色     | float32|
| size  | フォントサイズ | float32|
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| TE    | TE Tag                 | string |
| x     | X coord of start point | float32|
| y     | Y coord of start point | float32|
| text  | Text String            | string |
| color | Text Color             | string |
| size  | Font Size              | float32|
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
TE,20,20,Hello World!!,green,18.0
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
TE,20,20,Hello,World,black,3
```
@end

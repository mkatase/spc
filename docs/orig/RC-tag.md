@note
- Edit Time: 2026-02-14 14:45 First Edit
@end
@jp
## RCタグ
- カスタムカラーを設定できるタグ

@end
@en
## RC Tag
- Custom color Tag

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
|RC| name, color(Hexadecimal) |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| RC    | RCタグ              | string |
| name  | カスタム色名        | string |
| value | 16進数(RBG)         | string |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| RC    | RC Tag            | string |
| name  | Custom color name | string |
| value | Hexadecimal(RBG)  | string |
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
RC,kyara,#d8a373
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
RC,test,3a3a4b  # lack of '#'
```
@end

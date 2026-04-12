@note
- Edit Time: 2026-02-14 23:25 First Edit
@end
@jp
## ROタグ
- タグをまとめて回転処理できるタグ
- PL/RPタグとセットで使用する

@end
@en
## RO Tag
- Batch rotated processing tag
- Combined PL/PR tag

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
|GR| marker, angle |
|GR| E(preservation) |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| RO    | ROタグ         | string |
| marker| 設定マーカー   | string |
| angle | 角度           | float32|
| E     | エンドマーカー | string |
- 設定マーカーは＠マークから開始(@xxx)
- xxx部に文字数制限はないが、簡潔がよい
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| RO    | RO Tag       | string |
| marker| Group Marker | string |
| angle | Angle        | float32|
| E     | End Markder  | string |
- 1st char of Group Marker is '@' (@xxx)
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
RO,@R1,60
LN,0,0,10,10
CI,20,20,5
RO,E
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
RO,@R1,white  # <= Not angle
LN,0,0,10,10
CI,20,20,5
#            # <= Not End Marker
TE,20,20,Hello,World,black,3
```
@end

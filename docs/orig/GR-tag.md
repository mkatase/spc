@note
- Edit Time: 2026-02-14 23:10 First Edit
@end
@jp
## GRタグ
- タグをまとめて処理できるタグ
- PL/RPタグとセットで使用する

@end
@en
## GR Tag
- Batch processing tag
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
|GR| marker |
|GR| E(preservation) |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| GR    | GRタグ         | string |
| marker| 設定マーカー   | string |
| E     | エンドマーカー | string |
- 設定マーカーは＠マークから開始(@xxx)
- xxx部に文字数制限はないが、簡潔がよい
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| GR    | GR Tag       | string |
| marker| Group Marker | string |
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
GR,@G1
LN,0,0,10,10
CI,20,20,5
GR,E
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
GR,@G1
LN,0,0,10,10
CI,20,20,5
#            # <= Not End Marker
TE,20,20,Hello,World,black,3
```
@end

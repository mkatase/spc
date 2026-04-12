@note
- Edit Time: 2026-02-14 10:45 First Edit
@end
@jp
## VNタグ
- データ形式のバージョンを示すタグ
- 省略可能。デフォルトは、1。

@end
@en
## VN Tag
- Version Tag
- This tag is optional. Default is 1.

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
|VN| ver |

@end
@jp
# 要素説明
|要素|説明|タイプ|
|:-:|:-|-:|
| VN  | VNタグ              | string |
| ver | バージョン          | int32  |
@end
@en
# Element Description
|Element|Description|Type|
|:-:|:-|-:|
| VN  | VN Tag              | string |
| ver | Version Number      | int32  |
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
VN,1
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
VN,1.2
```
@end

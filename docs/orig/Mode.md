@note
- Edit Time: 2026-02-25 12:20 First Edit
@end
@jp
## モード説明
- 実行時に、モードを指定することで、多様が動作が可能となります。

@end
@en
## Mode Description
- By specifying a mode at execution, various operations are available depending on your purpose.
@end
#----------------------------------------------------------
@jp
# 使用方法

@end
@en
# Usage

@end
@common

```bash
$ spc [Mode] [Input File]
$ spc check test.dat
$ spc view test.dat
$ spc run test.dat
$ spc image test.dat
$ spc svg test.dat
```

@end
#----------------------------------------------------------
@jp
# モード一覧

| モード| 説明                            |
|:------|:--------------------------------|
| check | データの構文チェック            |
| view  | 画像のプレビュー表示            |
| run   | 画像の出力(webp/png)            |
| image | 画像の出力(低負荷/一部制限あり) |
| svg   | SVGファイルの出力               |

@end
@en
# Ordering
| Mode  | Description                                    |
|:------|:-----------------------------------------------| 
| check | Syntax check of input data                     |
| view  | Preview image display                          |
| run   | Output image files (webp/png )                 |
| image | Output image files (Low load/Some limitations) |
| svg   | Output SVG files only                          |

@end
#----------------------------------------------------------
@jp
# 各モード詳細

## チェックモード (check)
- 入力データの形式（フォーマット）を簡易的にチェックするモードです。

## ビューモード (view)
- 入力データに基づいた画像をウィンドに表示します。

## ランモード (run)
- 入力データに基づいた画像を出力します。
- 出力形式は、入力データ内で指定したwebp、または、pngとなります。
- 本モードでは、SVGは出力されません。
− 本モードでは、テキストの回転はできません。また、テキストの回転には対応していません。
- 処理の過程で、一瞬ウィンドウが表示されますが、これは動作仕様であり、異常ではありません。

## イメージモード (image)
- 入力データに基づいた画像を出力します。
- 出力形式は、入力データ内で指定したwebp、または、pngとなります。
- 本モードでは、SVGは出力されません。
− 本モードでは、テキストの回転はできません。
- 本モードは、軽量な描画エンジンを使用していますが、AR（円弧）タグが、荒い出力になる制限があります。

## SVGモード (svg)
- 入力データに基づいた画像を出力します。
- 本モードでのみ、SVG形式の出力が可能です。
- 本モードでのみ、テキストの回転(ROタグ併用)が可能です。

@end
@en
# Mode Details

## Check Mode (check)
- This mode performs a simple syntax and format check on the input data.

## View Mode (view)
- Displays the image generated from the input data in a preview window.

## Run Mode (run)
- Exports images based on the input data.
- The output format will be either WebP or PNG as specified in the input file.
- SVG output and text rotation are not supported in this mode.
- A window may appear briefly during processing; this is a known behavior and not a malfunction.
- Text rotation is not supported.
- SVG output is not supported.

## Image Mode (image)
- Exports images based on the input data.
- This mode uses a lightweight engine, but the AR (Arc) tag output will be rendered with coarse edges.
- Text rotation is not supported.
- SVG output is not supported.

## Svg Mode (svg)
- Exports images based on the input data.
- Only this mode supports SVG file output.
- Only this mode supports text rotation (when used with the RO tag).

@end

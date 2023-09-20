https://learn.microsoft.com/ja-jp/windows/win32/api/dwrite_3/nf-dwrite_3-idwritefactory6-getsystemfontcollection

```sh
cargo run --example font_collection2
```

```cpp
HRESULT GetSystemFontCollection(
        BOOL                     includeDownloadableFonts,
        DWRITE_FONT_FAMILY_MODEL fontFamilyModel,
  [out] IDWriteFontCollection2   **fontCollection
);
```

* SIMULATIONS を除く．
* `checkForUpdates`: `false` or `true`
  * 一度， `GetSystemFontCollection()` の実行のみ行う．
* `includeDownloadableFonts`: `false` or `true`
* `DWRITE_FONT_FAMILY_MODEL `: `DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC` or `DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE`
  * `typo` or `wss` と略す．

`includeDownloadableFonts == true` のとき，フォントリソースにアクセスできないフォントが存在する．
このフォントは`includeDownloadableFonts == false`のときは列挙されない．

```
フォント リソースがリモートのため、フォント リソースにアクセスできませんでした。 (0x8898500D) (Error)
```

`wss` よりも `typo` のファミリ数が少ない．
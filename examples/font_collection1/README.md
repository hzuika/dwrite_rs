https://learn.microsoft.com/ja-jp/windows/win32/api/dwrite_3/nf-dwrite_3-idwritefactory3-getsystemfontcollection

```sh
cargo run --example font_collection1
```

```cpp
HRESULT GetSystemFontCollection(
  BOOL                   includeDownloadableFonts,
  IDWriteFontCollection1 **fontCollection,
  BOOL                   checkForUpdates
);
```

* SIMULATIONS を除く．
* `includeDownloadableFonts`: `false` or `true`
* `checkForUpdates`: `false` or `true`

`includeDownloadableFonts == true` のとき，フォントリソースにアクセスできないフォントが存在する．
このフォントは`includeDownloadableFonts == false`のときは列挙されない．

```
フォント リソースがリモートのため、フォント リソースにアクセスできませんでした。 (0x8898500D) (Error)
```
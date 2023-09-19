https://learn.microsoft.com/ja-jp/windows/win32/api/dwrite/nf-dwrite-idwritefactory-getsystemfontcollection

```sh
cargo run --example font_collection
```

```cpp
HRESULT GetSystemFontCollection(
  [out] IDWriteFontCollection **fontCollection,
        BOOL                  checkForUpdates
);
```

* SIMULATIONS を除く．
* `checkForUpdates`: `false` or `true`
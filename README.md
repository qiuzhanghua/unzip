# unzip

unicode not supported?

```bash
cargo run
```
should be
```text
File 0 extracted to "中文/"
File 1 extracted to "中文/.DS_Store" (6148 bytes)
File 2 extracted to "__MACOSX/中文/._.DS_Store" (120 bytes)
File 3 extracted to "中文/folder/"
File 4 extracted to "中文/目录/"
```

instead of
```text
File 0 extracted to "Σ╕¡µûç/"
File 1 extracted to "Σ╕¡µûç/.DS_Store" (6148 bytes)
File 2 extracted to "__MACOSX/Σ╕¡µûç/._.DS_Store" (120 bytes)
File 3 extracted to "Σ╕¡µûç/folder/"
File 4 extracted to "Σ╕¡µûç/τ¢«σ╜ò/"
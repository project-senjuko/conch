cd ..

cargo clean | cargo build --release

"编译完成，按任意键退出..."
[Console]::ReadKey() | Out-Null

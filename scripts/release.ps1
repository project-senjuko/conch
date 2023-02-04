cd ..

cargo clean | cargo build --bin conch --release

"编译完成，按任意键退出..."
[Console]::ReadKey() | Out-Null

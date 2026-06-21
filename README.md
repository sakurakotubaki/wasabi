# OS

cargo buildするところ

```shell
rustup target add x86_64-unknown-uefi
```

ビルドを通す

```shell
cargo build --target x86_64-unknown-uefi
```
バイナリを調べる。

```shell
file target/x86_64-unknown-uefi/debug/wasabios.efi
```

QEMU(ケム)のバージョン確認

```shell
qemu-system-x86_64 --version
```

- diriveを実行してエミュレーターを起動するコマンド

```shell
qemu-system-x86_64 \
  -drive if=pflash,format=raw,readonly=on,file=third_party/ovmf/RELEASEX64_OVMF.fd \
  -drive if=pflash,format=raw,file=third_party/ovmf/OVMF_VARS.fd \
  -device qemu-xhci \
  -drive if=none,id=stick,format=raw,file=fat:rw:mnt \
  -device usb-storage,drive=stick \
  -boot order=d
```

buildした後にコピーをする場合

```shell
mkdir -p mnt/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/wasabios.efi mnt/EFI/BOOT/BOOTX64.EFI
```

UEFI Shell に落ちたときのために、`startup.nsh` も置く

```shell
printf 'fs0:\\EFI\\BOOT\\BOOTX64.EFI\r\n' > mnt/startup.nsh
```
# wasabi

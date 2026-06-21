.PHONY: build
build:
	cargo build --target x86_64-unknown-uefi
	mkdir -p mnt/EFI/BOOT
	cp target/x86_64-unknown-uefi/debug/wasabi.efi mnt/EFI/BOOT/BOOTX64.EFI

.PHONY: launch
launch:
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,readonly=on,file=third_party/ovmf/RELEASEX64_OVMF.fd \
		-drive if=pflash,format=raw,file=third_party/ovmf/OVMF_VARS.fd \
		-device qemu-xhci \
		-drive if=none,id=stick,format=raw,file=fat:rw:mnt \
		-device usb-storage,drive=stick \
		-boot order=d

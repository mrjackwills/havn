#!/bin/bash

UNAME_CMD="$(uname -m)"
case "$UNAME_CMD" in
x86_64) SUFFIX="x86_64" ;;
aarch64) SUFFIX="aarch64" ;;
armv6l) SUFFIX="armv6" ;;
esac

if [ -n "$SUFFIX" ]; then
	HAVN_GZ="havn_linux_${SUFFIX}.tar.gz"
	wget "https://github.com/mrjackwills/havn/releases/latest/download/${HAVN_GZ}"
	tar xzvf "${HAVN_GZ}" havn
	install -Dm 755 havn -t "${HOME}/.local/bin"
	rm "${HAVN_GZ}" havn
fi

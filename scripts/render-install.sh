#!/usr/bin/env bash
set -euxo pipefail

# shellcheck disable=SC2016
RTX_VERSION=$RTX_VERSION \
	RTX_CHECKSUM_LINUX_X86_64=$(grep "rtx-v.*linux-x64.tar.gz" "$RELEASE_DIR/$RTX_VERSION/SHASUMS256.txt") \
	RTX_CHECKSUM_LINUX_ARM64=$(grep "rtx-v.*linux-arm64.tar.gz" "$RELEASE_DIR/$RTX_VERSION/SHASUMS256.txt") \
	RTX_CHECKSUM_MACOS_X86_64=$(grep "rtx-v.*macos-x64.tar.gz" "$RELEASE_DIR/$RTX_VERSION/SHASUMS256.txt") \
	RTX_CHECKSUM_MACOS_ARM64=$(grep "rtx-v.*macos-arm64.tar.gz" "$RELEASE_DIR/$RTX_VERSION/SHASUMS256.txt") \
	envsubst '$RTX_VERSION,$RTX_CHECKSUM_LINUX_X86_64,$RTX_CHECKSUM_LINUX_ARM64,$RTX_CHECKSUM_MACOS_X86_64,$RTX_CHECKSUM_MACOS_ARM64' \
	<rtx/packaging/standalone/install.envsubst

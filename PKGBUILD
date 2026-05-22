# Maintainer: Karso Cheung <karsoo2023@gmail.com>

pkgname=asus-sound-fix
pkgver=
pkgrel=1
pkgdesc='asus vivobook14 flip no sound issue fixed'
url=''
license=()
makedepends=('cargo')
depends=()
arch=('x86_64')
source=()
b2sums=()

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target host-tuple
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    # for custom license, e.g. MIT
    # install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
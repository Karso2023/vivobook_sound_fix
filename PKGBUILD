# Maintainer: Karso Cheung <karsoo2023@gmail.com>
pkgname=sound-fix
pkgver=VERSION
pkgrel=1
pkgdesc="package to fix ASUS Vivobook 14 Flip (TP3407SA) no sound issue"
arch=('x86_64')
url="https://github.com/Karso2023/vivobook_sound_fix"
license=('GPL')
groups=('')
depends=('git')
makedepends=('cargo')
optdepends=('')
source=(""{,.sig})
sha256sums=('')

prepare() {
    cd "${pkgname}-${pkgver}"

    cd sound-fix
    # to do after i build the actual fix 
}

build() {
    cd "${pkgname}-${pkgver}/sound-fix"
    export CARGO_TARGET_DIR=target 
    cargo build --frozen --release --features git
}

package() {
    
}
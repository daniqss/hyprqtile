# Maintainer: daniqss <danielqueijo14@gmail.com>
pkgname=hyprqtile-bin
pkgver=0.1.5
pkgrel=1
pkgdesc="Qtile-like workspaces and monitors management for Hyprland"
url="https://github.com/daniqss/hyprqtile"
license=("GPL-3.0")
arch=("x86_64")
provides=("hyprqtile")
conflicts=("hyprqtile")
depends=('gcc-libs' 'glibc')
source=("https://github.com/daniqss/hyprqtile/releases/download/v$pkgver/hyprqtile-$pkgver-x86_64.tar.gz")
sha256sums=("a0f194fcc536aae926020498ac72ea447b34b1c0de7e8e27c380581cdf883b80")

package() {
    install -Dm755 hyprqtile -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

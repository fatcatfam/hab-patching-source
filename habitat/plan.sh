pkg_name=knock-knock
pkg_origin=learn-chef
pkg_version="0.1.0"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_filename="${pkg_name}-${pkg_version}.tar.gz"
pkg_shasum="$(cat /src/CHECKSUM)"
pkg_build_deps=( core/rust )
pkg_deps=( core/gcc-libs )
pkg_bin_dirs=( bin )

do_download() {
    cp /src/${pkg_filename} $HAB_CACHE_SRC_PATH
}

do_build() {
    cd $HAB_CACHE_SRC_PATH/$pkg_name-$pkg_version
    patch -p1 < /src/$pkg_name-$pkg_version-ordering.patch
    cargo build --release
}

do_install() {
    cd $HAB_CACHE_SRC_PATH/$pkg_name-$pkg_version
    cp target/release/$pkg_name $pkg_prefix/bin
}
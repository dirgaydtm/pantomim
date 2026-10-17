#!/bin/sh
# Builds the apt and dnf repositories served from GitHub Pages.
# usage: build.sh <dir with the .deb and .rpm> <output dir> <base URL>
# The signing key must already be in the gpg keyring (only one secret key).
set -eu
packages=$1 out=$2 url=$3

mkdir -p "$out/deb" "$out/rpm"
cp "$packages"/*.deb "$out/deb/"
cp "$packages"/*.rpm "$out/rpm/"
gpg --armor --export > "$out/key.asc"
gpg --export > "$out/key.gpg"

# A flat apt repository: `deb [signed-by=…] <url>/deb ./`
cd "$out/deb"
dpkg-scanpackages --multiversion . > Packages
gzip -k9 Packages
apt-ftparchive -o APT::FTPArchive::Release::Origin=pantomim release . > Release
gpg --batch --yes --clearsign -o InRelease Release
gpg --batch --yes --armor --detach-sign -o Release.gpg Release
cd - > /dev/null

# A dnf repository; the packages are unsigned, the metadata is signed.
createrepo_c "$out/rpm"
gpg --batch --yes --armor --detach-sign "$out/rpm/repodata/repomd.xml"
cat > "$out/pantomim.repo" <<EOF
[pantomim]
name=pantomim
baseurl=$url/rpm
enabled=1
gpgcheck=0
repo_gpgcheck=1
gpgkey=$url/key.asc
EOF

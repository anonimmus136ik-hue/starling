#!/usr/bin/env bash
# Requires a disposable Android device accessible through adb, plus JDK 25.
set -euo pipefail
source_dir=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
remote=$(adb shell mktemp -d /data/local/tmp/hrumpet.XXXXXX | tr -d '\r')
[[ "$remote" == /data/local/tmp/hrumpet.* ]]
trap 'adb shell rm -rf "$remote" >/dev/null || true; rm -rf "$work"' EXIT
cd "$work"
abi=$(adb shell getprop ro.product.cpu.abi | tr -d '\r')
case "$abi" in
  x86_64) arch=x86_64; triple=x86_64-linux-android; digest=7fca862ee1b2d5fe23cd9c9c3d9b7ad3c241947ad1a6cc9464ef2e674867105d ;;
  arm64-v8a) arch=arm64; triple=aarch64-linux-android; digest=d3eb7afe2240c26728a1bb440502c5f18ac3883e932d202dd7f0c9bcbbce4c37 ;;
  *) echo "Unsupported ABI: $abi" >&2; exit 1 ;;
esac
curl -fsSL --retry 3 "https://github.com/AngelAuraMC/angelauramc-openjdk-build/releases/download/download_jre25/jre25-android-$arch.tar.xz" -o jre.tar.xz
echo "$digest  jre.tar.xz" | sha256sum -c -
mkdir jre
tar -xJf jre.tar.xz -C jre
cp "$(find "${ANDROID_NDK_HOME:?}" -path "*/sysroot/usr/lib/$triple/libc++_shared.so" -print -quit)" jre/lib/
curl -fsSL --retry 3 https://cdn.modrinth.com/data/1bokaNcj/versions/mtjQ3mVQ/xaerominimap-forge-26.3-26.5.3.jar -o xaero.jar
echo "c4443a406b5c9c309548a110e5ae119af3a8df544522053e4249d8bbfb777ec3f9a85e1c0c9847771fa572c8c749814cb335dc8f62f9b529a3a2d00358c38b86  xaero.jar" | sha512sum -c -
cat > Probe.java <<'JAVA'
import xaero.common.minimap.waypoints.WaypointUtil;
public class Probe {
    public static void main(String[] args) throws ReflectiveOperationException {
        if (args.length != 0) {
            // Probe linking separately from execution of the self-contained helper.
            try {
                Class.forName("xaero.minimap.XaeroMinimapForge");
                System.out.println("Xaero entry class initialized");
            } catch (LinkageError missing) {
                missing.printStackTrace(System.out);
            }
            return;
        }
        for (int width : new int[]{1, 8, 16, 24, 40}) {
            System.out.println(width + " -> " + WaypointUtil.getAddedMinimapIconFrame(width));
        }
    }
}
JAVA
javac -cp xaero.jar Probe.java
java -cp .:xaero.jar Probe > reference.txt
compiler=$(find "$ANDROID_NDK_HOME" -name "${triple}28-clang" -print -quit)
"$compiler" -Wall -Wextra -Werror -I"$JAVA_HOME/include" -I"$JAVA_HOME/include/linux" "$source_dir/host.c" -L jre/lib/server -ljvm -Wl,--allow-shlib-undefined -o hrumpet-host
# Android's shell SELinux domain can write regular files here but cannot make symlinks.
tar --hard-dereference --dereference -cf - jre hrumpet-host xaero.jar Probe.class | adb shell -T "cd $remote && tar xf -"
run_probe() {
    adb shell "cd $remote && LD_LIBRARY_PATH=$remote/jre/lib:$remote/jre/lib/server JAVA_HOME=$remote/jre HOME=$remote TMPDIR=$remote ./hrumpet-host $remote/jre .:xaero.jar Probe $1"
}
run_probe '' | tr -d '\r' > android.txt
diff -u reference.txt android.txt
cat android.txt
run_probe link

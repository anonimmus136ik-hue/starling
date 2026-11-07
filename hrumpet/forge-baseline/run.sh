#!/usr/bin/env bash
# Run in an empty scratch directory, never in a player's instance.
set -euo pipefail
work=$(mktemp -d)
trap 'if [[ -n "${client_pid:-}" ]]; then kill -- "-$client_pid" 2>/dev/null || true; fi; rm -rf "$work"' EXIT
cd "$work"
curl --fail --silent --show-error --location --retry 3 https://maven.minecraftforge.net/net/minecraftforge/forge/26.3-66.0.2/forge-26.3-66.0.2-mdk.zip -o forge.zip
echo 'b8b5f61a5b140439f1b3febb117b61fbb8d93b5017e039af081f8f8e71660f3f  forge.zip' | sha256sum -c -
unzip -q forge.zip
mkdir -p run/mods
jar=run/mods/xaerominimap-forge-26.3-26.5.3.jar
curl --fail --silent --show-error --location --retry 3 https://cdn.modrinth.com/data/1bokaNcj/versions/mtjQ3mVQ/xaerominimap-forge-26.3-26.5.3.jar -o "$jar"
echo "c4443a406b5c9c309548a110e5ae119af3a8df544522053e4249d8bbfb777ec3f9a85e1c0c9847771fa572c8c749814cb335dc8f62f9b529a3a2d00358c38b86  $jar" | sha512sum -c -
setsid xvfb-run -a -s '-screen 0 1280x720x24' bash gradlew --no-daemon --console=plain runClient > client.log 2>&1 &
client_pid=$!
for ((i=0;i<240;i++)); do
    if grep -Eiq "(Xaero.s Minimap: (Stage 2|.*loaded)|Loading Xaero.s Minimap.*Stage 2)" run/logs/latest.log 2>/dev/null; then
        grep -Ei "Xaero.s Minimap.*(stage|loaded)" run/logs/latest.log | tail -10
        echo "Observed Xaero client initialization in reference Forge."
        exit 0
    fi
    if ! kill -0 "$client_pid" 2>/dev/null; then break; fi
    sleep 5
done
tail -100 client.log
if [[ -f run/logs/latest.log ]]; then tail -100 run/logs/latest.log; fi
echo "Xaero initialization was not confirmed." >&2
exit 1

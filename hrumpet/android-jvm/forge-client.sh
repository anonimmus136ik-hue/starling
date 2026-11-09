#!/usr/bin/env bash
# Called from probe.sh with the prepared Android VM and a private disposable profile.
set -euo pipefail
remote=${1:?Android scratch directory}
mod=${2:?Forge mod JAR}
game="$PWD/game"
mkdir -p "$game/versions/26.3" "$game/mods"
cp "$mod" "$game/mods/"
cp version.json "$game/versions/26.3/26.3.json"
echo '{}' > "$game/launcher_profiles.json"
fetch() {
    curl -fsSL --retry 3 "$1" -o "$3"
    echo "$2  $3" | sha1sum -c -
}
read -r url hash < <(jq -r '.downloads.client | [.url,.sha1] | @tsv' version.json)
fetch "$url" "$hash" "$game/versions/26.3/26.3.jar"
curl -fsSL --retry 3 https://maven.minecraftforge.net/net/minecraftforge/forge/26.3-66.0.2/forge-26.3-66.0.2-installer.jar -o installer.jar
echo '7ce840f220dc7897fb66541c6d856294c393fc86776ca50e903ad0461e06d984  installer.jar' | sha256sum -c -
java -Djava.awt.headless=true -jar installer.jar --installClient "$game"
unzip -p installer.jar version.json > forge.json
# Match the inherited launcher class path, giving Forge precedence over vanilla versions.
python3 - <<'PY' > classpath.tsv
import json
seen = set()
for profile in ("forge.json", "version.json"):
    with open(profile) as f:
        libraries = json.load(f)["libraries"]
    for lib in libraries:
        coords = lib["name"].split(":")
        key = tuple(coords[:2] + coords[3:])
        if key in seen or len(coords) > 3 and coords[3].startswith("natives-"):
            continue
        rules = lib.get("rules", [])
        if rules:
            allowed = False
            for rule in rules:
                if rule.get("os", {}).get("name", "linux") == "linux":
                    allowed = rule["action"] == "allow"
            if not allowed:
                continue
        art = lib["downloads"]["artifact"]
        seen.add(key)
        print(art["path"], art["sha1"], art["url"] or "-", sep="\t")
PY
mkdir -p runtime/libraries
classpath=
while read -r path hash url; do
    dest="runtime/libraries/$path"
    mkdir -p "$(dirname "$dest")"
    if [[ -f "$game/libraries/$path" ]]; then
        cp "$game/libraries/$path" "$dest"
        echo "$hash  $dest" | sha1sum -c -
    else
        fetch "$url" "$hash" "$dest"
    fi
    classpath="${classpath:+$classpath:}$remote/runtime/libraries/$path"
done < classpath.tsv
cp -r "$game/mods" runtime/
tar -cf - runtime | adb shell -T "cd $remote && tar xf -"
adb shell "cd $remote/runtime && HOME=$remote TMPDIR=$remote JAVA_HOME=$remote/jre LD_LIBRARY_PATH=$remote/jre/lib:$remote/jre/lib/server $remote/hrumpet-host $remote/jre $classpath net/minecraftforge/bootstrap/ForgeBootstrap --launchTarget forge_client --version 26.3 --gameDir $remote/runtime --assetsDir $remote/runtime/assets --assetIndex 26.3 --username Player --accessToken 0 --demo" 2>&1 | tee forge-android.log
grep -F "Loading Xaero's Minimap - Stage 2/3" forge-android.log

#!/usr/bin/env bash
set -eu
cd "$(dirname "$0")"
mkdir -p build/host build/plugin/META-INF/services
javac -d build/host FrameHost.java
javac -d build/plugin demo/Square.java
echo demo.Square > build/plugin/META-INF/services/java.util.function.Consumer
jar --create --file build/square.jar -C build/plugin .
g++ -std=c++17 -Wall -Wextra -Werror host.cpp -I"$JAVA_HOME/include" -I"$JAVA_HOME/include/linux" -L"$JAVA_HOME/lib/server" -Wl,-rpath,"$JAVA_HOME/lib/server" -ljvm -o build/hrumpet
build/hrumpet build/host build/square.jar build/frame.ppm
python3 - <<'PY'
from pathlib import Path
img=Path('build/frame.ppm').read_bytes()
expected=b'P6\n64 64\n255\n'+b''.join(bytes.fromhex('5de1b7') if 16<=x<48 and 16<=y<48 else bytes(3) for y in range(64) for x in range(64))
assert img==expected
PY
jar --create --file build/unsupported.jar -C build/host .
if build/hrumpet build/host build/unsupported.jar build/rejected.ppm; then exit 1; fi

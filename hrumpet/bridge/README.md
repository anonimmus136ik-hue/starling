# Hrumpet framebuffer bridge
Linux + JDK 25 + g++: export JAVA_HOME to the JDK, then run `bash test.sh`.
The C++ host starts a JVM and loads a separate trusted JAR. Each provider implements Consumer<int[]> and writes one RGB frame (64×64). The test compares every output pixel to the specified image and rejects a JAR with no provider. Source code is not loaded from the game or the Xaero archive.
This is a native/JVM interoperability experiment. No Forge event bus, Minecraft classes, Mixin transformer, Android JVM embedding, or in-game frame presentation is implemented. The original Xaero JAR cannot be played with this host.

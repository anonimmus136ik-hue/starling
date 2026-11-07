# Hrumpet: Java mod compatibility investigation

Target: run the user's unmodified Xaero's Minimap Forge 26.5.3 JAR inside the Horizon Rust client for Minecraft 26.3. A native minimap is not an acceptance test.

The supplied archive declares Forge 66.0.2 or newer and contains 661 class files and four Mixin configs. An Android file picker can supply the bytes. The regular mod installer subsequently calls horizon_mods::Package::read and expects a Horizon manifest/component. Selecting a JAR does not execute its classes.

A proposed Java host needs class loading for the Minecraft version, Forge lifecycle and mappings, translated game hooks for injected methods, and rendering/input adapters. A list of nearby entities passed over JNI would cover only one input. No runtime compatibility is implemented in this branch.

First executable milestone: load a small unmodified Forge test JAR, deliver its client initialization and a frame event from Horizon, and observe HUD pixels produced by its Java callback. Next test the target JAR unchanged and record the first unsupported class or injection target. Test multiplayer and compare frame times on the same Android device, world and settings. Branding alone cannot establish compatibility.

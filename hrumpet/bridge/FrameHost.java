package hrumpet;
import java.net.URLClassLoader;
import java.nio.file.Path;
import java.util.ServiceLoader;
import java.util.function.Consumer;

/** Desktop vertical slice. Only run trusted plugins: the JVM is not a sandbox. */
public final class FrameHost implements AutoCloseable {
    private final URLClassLoader loader;
    private final Consumer<int[]> renderer;
    @SuppressWarnings("unchecked")
    public FrameHost(String jar) throws Exception {
        loader = new URLClassLoader(new java.net.URL[]{Path.of(jar).toUri().toURL()}, FrameHost.class.getClassLoader());
        try {
            var providers = ServiceLoader.load(Consumer.class, loader).stream().toList();
            if (providers.size() != 1) throw new IllegalArgumentException("Expected one Hrumpet framebuffer provider; Forge lifecycle and Mixins are unsupported");
            renderer = (Consumer<int[]>)providers.get(0).get();
        } catch (Throwable e) { loader.close(); throw e; }
    }
    public int[] frame() { int[] pixels = new int[64*64]; renderer.accept(pixels); return pixels; }
    public void close() throws Exception { loader.close(); }
}

package demo;
import java.util.function.Consumer;
public final class Square implements Consumer<int[]> {
    public void accept(int[] pixels) {
        for (int y=16; y<48; y++)
            for (int x=16; x<48; x++) pixels[y*64+x]=0x5de1b7;
    }
}

#include <jni.h>
#include <array>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>

static void check(JNIEnv* env) {
    if (env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
        throw std::runtime_error("JAR execution failed");
    }
}
int main(int argc, char** argv) {
    if (argc != 4) { std::cerr << "Usage: hrumpet HOST_CLASSES PLUGIN.jar OUTPUT.ppm\n"; return 2; }
    std::string path = std::string("-Djava.class.path=") + argv[1];
    JavaVMOption opt{path.data(), nullptr};
    JavaVMInitArgs args{JNI_VERSION_1_8, 1, &opt, JNI_FALSE};
    JavaVM* vm = nullptr;
    JNIEnv* env = nullptr;
    if (JNI_CreateJavaVM(&vm, reinterpret_cast<void**>(&env), &args) != JNI_OK) return 2;
    int result = 0;
    try {
        auto cls=env->FindClass("hrumpet/FrameHost"); check(env);
        auto init=env->GetMethodID(cls,"<init>","(Ljava/lang/String;)V"); check(env);
        auto file=env->NewStringUTF(argv[2]); check(env);
        auto host=env->NewObject(cls,init,file); check(env);
        auto draw=env->GetMethodID(cls,"frame","()[I"); check(env);
        auto frame=static_cast<jintArray>(env->CallObjectMethod(host,draw)); check(env);
        std::array<jint,4096> pixels{};
        env->GetIntArrayRegion(frame,0,pixels.size(),pixels.data()); check(env);
        auto close=env->GetMethodID(cls,"close","()V"); check(env);
        env->CallVoidMethod(host,close); check(env);
        std::ofstream out(argv[3],std::ios::binary);
        out.exceptions(std::ios::badbit | std::ios::failbit);
        out<<"P6\n64 64\n255\n";
        for(auto c:pixels) { out.put(char((c>>16)&255)); out.put(char((c>>8)&255)); out.put(char(c&255)); }
        out.close();
    } catch(const std::exception& e) { std::cerr << e.what() << '\n'; result=1; }
    vm->DestroyJavaVM();
    return result;
}

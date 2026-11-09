// Launch an OpenJDK main class from a native Android process (outside ART).
#include <jni.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char *property(const char *key, const char *value) {
    size_t n = strlen(key) + strlen(value) + 1;
    char *result = malloc(n);
    if (result) snprintf(result, n, "%s%s", key, value);
    return result;
}

int main(int argc, char **argv) {
    if (argc < 4) {
        fprintf(stderr, "Usage: hrumpet-host JAVA_HOME CLASSPATH MAIN/CLASS [ARGS...]\n");
        return 2;
    }
    JavaVMOption options[2] = {
        {property("-Djava.home=", argv[1]), NULL},
        {property("-Djava.class.path=", argv[2]), NULL}
    };
    if (!options[0].optionString || !options[1].optionString) return 1;
    JavaVMInitArgs init = {JNI_VERSION_1_8, 2, options, JNI_FALSE};
    JavaVM *vm = NULL;
    JNIEnv *env = NULL;
    jint status = JNI_CreateJavaVM(&vm, (void **)&env, &init);
    free(options[0].optionString);
    free(options[1].optionString);
    if (status != JNI_OK) {
        fprintf(stderr, "OpenJDK startup failed (%d)\n", status);
        return 1;
    }
    int result = 1;
    jclass main_class = (*env)->FindClass(env, argv[3]);
    if (!main_class) goto done;
    jmethodID entry = (*env)->GetStaticMethodID(env, main_class, "main", "([Ljava/lang/String;)V");
    if (!entry) goto done;
    jclass string = (*env)->FindClass(env, "java/lang/String");
    if (!string) goto done;
    jobjectArray args = (*env)->NewObjectArray(env, argc - 4, string, NULL);
    if (!args) goto done;
    for (int i = 4; i < argc; i++) {
        jstring arg = (*env)->NewStringUTF(env, argv[i]);
        if (!arg) goto done;
        (*env)->SetObjectArrayElement(env, args, i - 4, arg);
        (*env)->DeleteLocalRef(env, arg);
        if ((*env)->ExceptionCheck(env)) goto done;
    }
    (*env)->CallStaticVoidMethod(env, main_class, entry, args);
    result = (*env)->ExceptionCheck(env) ? 1 : 0;
done:
    if ((*env)->ExceptionCheck(env)) (*env)->ExceptionDescribe(env);
    if ((*vm)->DestroyJavaVM(vm) != JNI_OK) result = 1;
    return result;
}

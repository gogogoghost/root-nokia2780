#ifdef __ANDROID__

#define SOCKET_PATH "/dev/socket/su-daemon"
#define SHELL_PATH "/system/bin/sh"

#else

#define SOCKET_PATH "/tmp/su-daemon"
#define SHELL_PATH "/usr/bin/sh"

#endif
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <errno.h>
#include <string.h>
#include <sys/select.h>
#include "const.h"

void error(const char *msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

int main() {
    int sockfd;
    struct sockaddr_un serv_addr;
    ssize_t numBytes;
    char buffer[4096];

    // create socket
    sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sockfd == -1) {
        error("ERROR opening socket");
        return -1;
    }

    // address
    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sun_family = AF_UNIX;
    strncpy(serv_addr.sun_path, SOCKET_PATH, sizeof(serv_addr.sun_path) - 1);

    // connect
    if (connect(sockfd, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) == -1) {
        error("ERROR connecting to socket");
        return -2;
    }

    printf("Connected to %s\n", SOCKET_PATH);

    while (1) {
        fd_set read_fds;
        FD_ZERO(&read_fds);
        FD_SET(sockfd, &read_fds);
        FD_SET(STDIN_FILENO, &read_fds);

        // 使用 select 监听套接字和标准输入
        if (select(sockfd + 1, &read_fds, NULL, NULL, NULL) == -1) {
            error("ERROR in select");
            return -3;
        }

        // 从标准输入读取并发送到套接字
        if (FD_ISSET(STDIN_FILENO, &read_fds)) {
            numBytes = read(STDIN_FILENO, buffer, sizeof(buffer));
            if (numBytes <= 0) {
                break;
            }
            if (write(sockfd, buffer, numBytes) == -1) {
                error("ERROR writing to socket");
                return -4;
            }
        }

        // 从套接字读取并输出到标准输出
        if (FD_ISSET(sockfd, &read_fds)) {
            numBytes = read(sockfd, buffer, sizeof(buffer));
            if (numBytes <= 0) {
                break;
            }
            if (write(STDOUT_FILENO, buffer, numBytes) == -1) {
                error("ERROR writing to stdout");
                return -5;
            }
        }
    }

    // 关闭套接字
    close(sockfd);

    printf("Disconnected from %s\n", SOCKET_PATH);

    return 0;
}
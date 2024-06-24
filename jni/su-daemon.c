#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <string.h>
#include <fcntl.h>
#include "const.h"

#define BACKLOG 5

void handle_client(int client_socket) {
    // fork child process
    pid_t pid = fork();
    if (pid == -1) {
        perror("fork");
        close(client_socket);
        exit(EXIT_FAILURE);
    } else if (pid == 0) {
        //child process
        dup2(client_socket, 0);  // stdin
        dup2(client_socket, 1);  // stdout
        dup2(client_socket, 2);  // stderr
        
        //run sh -i
        execl(SHELL_PATH, SHELL_PATH, "-i",NULL);
        
        //unexpect
        perror("execl");
        exit(EXIT_FAILURE);
    } else {
        //do nothing
    }
}

int main() {

    int server_socket, client_socket;
    struct sockaddr_un server_addr, client_addr;
    socklen_t client_addr_len;

    // create socket
    server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_socket == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    // set address
    server_addr.sun_family = AF_UNIX;
    strncpy(server_addr.sun_path, SOCKET_PATH, sizeof(server_addr.sun_path) - 1);
    unlink(SOCKET_PATH);  // 删除可能存在的旧套接字文件

    // bind
    if (bind(server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr)) == -1) {
        perror("bind");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    // listen
    if (listen(server_socket, BACKLOG) == -1) {
        perror("listen");
        close(server_socket);
        exit(EXIT_FAILURE);
    }

    printf("su-daemon listening on %s\n", SOCKET_PATH);

    while (1) {
        // accept
        client_addr_len = sizeof(client_addr);
        client_socket = accept(server_socket, (struct sockaddr *)&client_addr, &client_addr_len);
        if (client_socket == -1) {
            perror("accept");
            continue;
        }
        printf("Client connected\n");
        handle_client(client_socket);
    }
    close(server_socket);

    return 0;
}
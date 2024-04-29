import socket
from multiprocessing import Pool

if __name__ == '__main__':
    s = socket.socket(socket.AddressFamily.AF_INET, socket.SocketKind.SOCK_STREAM)
    s.bind(("0.0.0.0", 7))
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.listen(0)
    print(f"{s=}")
    while True:
        print("Waiting for connection...")
        client_socket, client_address = s.accept()
        print(f"{client_socket=}\n{client_address=}")

        while True:
            msg = client_socket.recv(1024)
            try:
                text =f"{msg.decode('utf-8')=}"
                print(text)
            except UnicodeDecodeError:
                pass
            if len(msg) == 0:
                client_socket.close()
                break
            client_socket.send(msg)
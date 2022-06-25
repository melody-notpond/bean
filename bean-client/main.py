import socket

SERVER_ADDRESS = ('localhost', 6429)

if __name__ == '__main__':
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect(SERVER_ADDRESS)
        sock.sendall(b'uwu')
        data = sock.recv(1024)
        print(data)


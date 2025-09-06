import socket

HOST = '127.0.0.1'  
PORT = 4538       

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))  
    print(f"Connected to {HOST}:{PORT}")

    message = "CREATE TABLE status (id INTEGER, name TEXT)\n" 
    s.sendall(message.encode())       

    data = s.recv(1024)
    print("Received:", data.decode())

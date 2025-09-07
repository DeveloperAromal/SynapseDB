import socket

HOST = '127.0.0.1'  
PORT = 4538       

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))  
    print(f"Connected to {HOST}:{PORT}")

    message = "Create a table named is_deleted with two columns: id as an integer and name as text.\n" 
    s.sendall(message.encode())       

    data = s.recv(1024)
    print("Received:", data.decode())

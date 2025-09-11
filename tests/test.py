import socket
import time

HOST = '127.0.0.1'  
PORT = 4538       

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))  
    print(f"Connected to {HOST}:{PORT}")

    query = "show all data in users table"
    
    st = time.time()
    
    s.sendall(query.encode())       

    data = s.recv(1024)
    
    et = time.time()
    
    print(f"latency: ~ {(et - st) * 1000}ms")
    print("Received:", data.decode())

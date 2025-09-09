import socket
import time

HOST = '127.0.0.1'  
PORT = 4538       

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))  
    print(f"Connected to {HOST}:{PORT}")

    # message = "Create a table named users with two columns: id as an integer and name as text.\n" 
    
    
    # message = "Insert a row into users with id 3 and name 'Charlie. \n"
    message = "Show all rows from users. \n"
    
    st = time.time()
    
    s.sendall(message.encode())       

    data = s.recv(1024)
    
    et = time.time()
    
    print(f"latency: ~ {(et - st) * 1000}ms")
    print("Received:", data.decode())

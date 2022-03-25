# Rusty CSV Toy Protocol

## How to Run

1. Install Rust and Cargo.
2. cd to the folder.
3. `cargo run`.

Then use this Python script to test:

```python
import socket
import base64

csv = """
142, 160, 28, 10, 5, 3,  60, 0.28,  3167
175, 180, 18,  8, 4, 1,  12, 0.43,  4033
129, 132, 13,  6, 3, 1,  41, 0.33,  1471
138, 140, 17,  7, 3, 1,  22, 0.46,  3204
232, 240, 25,  8, 4, 3,   5, 2.05,  3613
135, 140, 18,  7, 4, 3,   9, 0.57,  3028
"""


enc = base64.b64encode(csv.encode())

len_enc = len(enc)

trailing = "0"*(12 - len(str(len_enc))) 

TCP_IP = '127.0.0.1'
TCP_PORT = 3333
BUFFER_SIZE = 1024
MESSAGE_SET = f"SET {trailing}{len_enc}".encode()
print(len(MESSAGE_SET))
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))
s.send(MESSAGE_SET)
data = s.recv(BUFFER_SIZE)
print("received data: ", data)

if data == b"\r":
	print(enc)
	s.send(enc)
data = s.recv(BUFFER_SIZE)
print("received data: ", data)

s.close()


```
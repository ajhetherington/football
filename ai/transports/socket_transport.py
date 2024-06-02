"""
InterProcessCommunication
Functions for reading & writing data to the football simulator over tcp sockets.

## Example
```python
from ai.transports.socket_transport import get_data
gen = get_data(21878)
for response in get_data(21878):
    print(response)
```

"""

import socket
import json
import struct
from time import sleep
from typing import Generator
from ai.transports import Transport
import logging
import os
socket.setdefaulttimeout(2)

logger = logging.getLogger(__name__)

END_OF_FILE = os.environ.get("END_OF_FILE", "EOF").encode('utf-8')


class TCP:
    def __init__(self, server_addr: str) -> None:
        self.server_addr = server_addr
        # sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        # self.socket = sock.bind(server_addr)
        pass
    
    def read_frame(self):
        while True:
            try:
                sock = new_poll_for_connection(self.server_addr)
                # sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                # read message size first, 8 bytes for u32
                size_buffer = memoryview(bytearray(4))
                # "!" means network(big-eindian)
                # "I" means unsigned int
                sock.recv_into(size_buffer, 4)
                size, = struct.unpack('!I', size_buffer)
                buffer = memoryview(bytearray(size))
                sock.recv_into(buffer, size)
            except (ConnectionRefusedError, ConnectionResetError) as e:
                # print(f"found error {e.__traceback__}")
                # logger.exception(e)
                continue
            sock.close()
            return json.loads(buffer.tobytes())
    
    def write_frame(self, data: str):
        sock = new_poll_for_connection(self.server_addr)
        # read message size first, 8 bytes for u32
        actions_bytes = data.encode('utf-8')
        message_len = len(actions_bytes)

        sock.sendall(struct.pack("!I", message_len))
        sock.sendall(actions_bytes)
        sock.close()



        


class TCPTransport(Transport):

    def __init__(self, host: str = 'localhost', port: int = 21878) -> None:
        self.host = host
        self.port = port
        # self.read_socket = threading.Thread(target=get_data, arg)
        # self.write_socket = threading.Thread(thread=)
        super().__init__()

    def read_frame(self) -> Generator:
        return super().read_frame()
    
    def write_frame(self) -> Generator:
        return super().write_frame()


    def frame_generator(self) -> Generator[dict|str|list, str, None]:
        # sock = _poll_for_connection(self.host, self.port)
        # sock.settimeout(30)

        i = 1
        while True:
            logger.info(i)
            i += 1
            print('starting conenction')
            sock = _poll_for_connection(self.host, self.port)
            print('after new connection')
            sock.settimeout(30)
            buffer = b''
            while True:
                try:
                    data: bytes = sock.recv(1024)
                except ConnectionResetError:
                    print("new connection")
                    break
                if data:
                    if END_OF_FILE in data:
                        # cutoff END_OF_FILE token
                        print('got end of file')
                        buffer += data[:(data.index(END_OF_FILE))]
                        break
                    buffer += data
                else:
                    break
            if buffer:
                decoded = buffer.decode("utf-8").strip()
                parsed = {}
                try:
                    jsn = json.loads(decoded)
                    print('found json')
                    parsed = jsn

                except json.decoder.JSONDecodeError as e:
                    logger.exception(e)
                    logger.warning("Couldn't deserialize json in tcp transport")
                    parsed = {}
                    pass
                
                message = yield parsed # read in next message
                logger.info(f'bout to send this message {message}')
                sock.sendall(message.encode('utf-8') + b'\n')
                logger.info(f"sent: {message.encode('utf-8') + b'\n'}")
                print('after close')
            


def new_poll_for_connection(addr: str) -> socket.socket:
    """
    Blocking in thread for a tcp socket connection
    """
    host, port = addr.split(':')
    while True:
        # socket.SOCK_STREAM: socket stream
        # SOCK_DGRAM: UDP socket
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        t = s.connect_ex((host, int(port)))
        if t == 0:
            # t == 0 means no error, successful connection
            return s
        else:
            sleep(0.05)



def _poll_for_connection(host: str, port: int) -> socket.socket:
    """
    Blocking in thread for a tcp socket connection
    """
    while True:
        # socket.SOCK_STREAM: socket stream
        # SOCK_DGRAM: UDP socket
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        t = s.connect_ex((host, port))
        if t == 0:
            # t == 0 means no error, successful connection
            # s.listen(1)
            return s
        else:
            sleep(0.01)



def get_data(port: int):
    print(port)
    while True:
        buffer = b""

        sock = _poll_for_connection("localhost", port)
        
        while True:
            data: bytes = sock.recv(4096)
            if data:
                if END_OF_FILE in data:
                    # cutoff END_OF_FILE token
                    buffer += data[:-(data.index(END_OF_FILE)) + 1]
                    break
                buffer += data
            else:
                break
        if buffer:
            decoded = buffer.decode("utf-8").strip()
            try:
                jsn = json.loads(decoded)
                print('found json')
                print(jsn)
                yield jsn
            except json.JSONDecodeError as e:
                print('found text')
                print(decoded)
                yield decoded
                pass

            
        try:
            sock.shutdown(socket.SHUT_RDWR)
            sock.close()
        except OSError as e:
            print('socket ')

# thread1 = threading.Thread(target=get_data, args=(6969,))
# thread1.start()

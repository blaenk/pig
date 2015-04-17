import zmq

context = zmq.Context()
socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:5555")

socket.send_multipart([b"ruby", b"puts 'testing'"])
highlighted = socket.recv()

print("reply was:\n{}".format(highlighted))


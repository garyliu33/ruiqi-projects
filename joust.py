from flask import Flask
from flask_socketio import SocketIO
from flask_cors import CORS
import pyvjoy

app = Flask(__name__)
CORS(app)
socketio = SocketIO(app, cors_allowed_origins="*")

j = pyvjoy.VJoyDevice(1)

@socketio.on('connect')
def handle_connect():
    print("Client connected")

@socketio.on('press_button')
def handle_press_button(data):
    try:
        btn = int(data.get('btn', 1))
        j.set_button(btn, 1)  # press and hold
        print(f"Pressed button {btn}")
    except Exception as e:
        print("Press error:", e)

@socketio.on('release_button')
def handle_release_button(data):
    try:
        btn = int(data.get('btn', 1))
        j.set_button(btn, 0)  # release
        print(f"Released button {btn}")
    except Exception as e:
        print("Release error:", e)

if __name__ == '__main__':
    socketio.run(app, host='0.0.0.0', port=5000)

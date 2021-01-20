#!/usr/bin/env python3

from flask import Flask, request
from multiprocessing import Process, Queue
import logging
import time

port = 8080 # TODO: set this from cmdline args

cmds = Queue()
quitting = Queue()

app = Flask(__name__)
@app.route('/', methods=['GET'])
def get_command():
	if cmds.empty():
		return ''
	else:
		return cmds.get()

@app.route('/', methods=['POST'])
def get_output():
	if request.data.decode() == "quitting":
		quitting.put(True)
	print("{} ".format(request.data.decode()),end='\n> ')
	return ''

if __name__ == "__main__":
	log = logging.getLogger('werkzeug')
	log.setLevel(logging.ERROR)
	
	t = Process(target=app.run, kwargs={'host': '0.0.0.0'})
	t.start()

	running = True
	while running:
		cmd = input("> ")
		if cmd == "kill" or cmd =="quit" or cmd == "exit":
			print("Waiting for agent to close...")
			cmds.put("quit")
			while quitting.empty():
				pass
			t.terminate()
			t.join()
			running = False
		else:
			cmds.put(cmd)
import serial
from pynput.keyboard import Controller
from time import sleep

ser = serial.Serial('COM9', 9600)
keyboard = Controller()

while True:
	line = ser.readline().decode().strip()
	if line == "down":
		print('down')
		keyboard.press("z")
		sleep(0.1)  # Hold key for 100 ms
		keyboard.release("z")
	if line == "up":
		print('up')
		keyboard.press("a")
		sleep(0.1)
		keyboard.release("a")
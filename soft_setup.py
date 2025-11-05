import subprocess
import time

A = r"C:\Users\simua\Documents\law.exe"
B = r"C:\Users\simua\Documents\ffb.exe"

# launch A
proc_a = subprocess.Popen([A], creationflags=subprocess.CREATE_NEW_CONSOLE)

# wait a fixed time for A to initialize
time.sleep(5)   # adjust seconds as needed

# launch B
proc_b = subprocess.Popen([B], creationflags=subprocess.CREATE_NEW_CONSOLE)

print("Started A (pid {}) and B (pid {})".format(proc_a.pid, proc_b.pid))
import mmap
import ctypes
import time
import requests
# ======= Shared Memory Definitions =======
class vec3(ctypes.Structure):
    _pack_ = 4
    _fields_ = [('x', ctypes.c_float), ('y', ctypes.c_float), ('z', ctypes.c_float)]
class acsVehicleInfo(ctypes.Structure):
    _pack_ = 4
    _fields_ = [
        ('carId', ctypes.c_int32),
        ('driverName', ctypes.c_char * 64),
        ('carModel', ctypes.c_char * 64),
        ('speedMS', ctypes.c_float),
        ('bestLapMS', ctypes.c_int32),
        ('lapCount', ctypes.c_int32),
        ('currentLapInvalid', ctypes.c_int32),
        ('currentLapTimeMS', ctypes.c_int32),
        ('lastLapTimeMS', ctypes.c_int32),
        ('worldPosition', vec3),
        ('isCarInPitline', ctypes.c_int32),
        ('isCarInPit', ctypes.c_int32),
        ('carLeaderboardPosition', ctypes.c_int32),
        ('carRealTimeLeaderboardPosition', ctypes.c_int32),
        ('spLineLength', ctypes.c_float),
        ('isConnected', ctypes.c_int32),
        ('suspensionDamage', ctypes.c_float * 4),
        ('engineLifeLeft', ctypes.c_float),
        ('tyreInflation', ctypes.c_float * 4),
    ]
class SPageFileSimHub(ctypes.Structure):
    _pack_ = 4
    _fields_ = [
        ('numVehicles', ctypes.c_int32),
        ('focusVehicle', ctypes.c_int32),
        ('serverName', ctypes.c_char * 512),
        ('vehicleInfo', acsVehicleInfo * 128),
        ('acInstallPath', ctypes.c_char * 512),
        ('isInternalMemoryModuleLoaded', ctypes.c_int32),
        ('pluginVersion', ctypes.c_char * 32)
    ]
# ======= Init Shared Memory =======
shared_mem = mmap.mmap(0, ctypes.sizeof(SPageFileSimHub), "acpmf_simhub_v2")
telemetry = SPageFileSimHub.from_buffer(shared_mem)
# ======= Config =======
SERVER_URL = "http://192.168.5.74:3000/send"
last_sent_data = None
# ======= Loop =======
while True:
    try:
        focus = telemetry.focusVehicle
        car = telemetry.vehicleInfo[focus]
        speed = round(car.speedMS, 2)
        driver = car.driverName.decode().strip()
        print(f"Car {car.carId} | Speed: {speed} m/s | Driver: {driver}")
        if speed is not None and speed != last_sent_data:
            # Serialize data to raw bytes using struct (example: speed and carId as float + int)
            payload = ctypes.string_at(ctypes.byref(car), ctypes.sizeof(acsVehicleInfo))
            try:
                response = requests.post(SERVER_URL, data=payload, headers={"Content-Type": "application/octet-stream"})
                print(f"📤 Sent {len(payload)} bytes | Response: {response.status_code}")
                last_sent_data = speed
            except requests.RequestException as e:
                print(f"❌ HTTP POST failed: {e}")
        time.sleep(0.05)
    except Exception as e:
        print(f"⚠️ Error in loop: {e}")
        time.sleep(1)

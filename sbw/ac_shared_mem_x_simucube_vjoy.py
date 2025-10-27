import time

"""
SimHub SharedMemory
Based on CrewChief work (Sparten)
No licence apply.
"""
import mmap
import os
import struct
import functools
import ctypes
from ctypes import c_float, c_char, c_int32
import pygame
import pyvjoy


class vec3(ctypes.Structure):
    _pack_ = 4
    _fields_ = [
        ("x", c_float),
        ("y", c_float),
        ("z", c_float),
    ]


class acsVehicleInfo(ctypes.Structure):
    _pack_ = 4
    _fields_ = [
        ("carId", c_int32),
        ("driverName", c_char * 64),
        ("carModel", c_char * 64),
        ("speedMS", c_float),
        ("bestLapMS", c_int32),
        ("lapCount", c_int32),
        ("currentLapInvalid", c_int32),
        ("currentLapTimeMS", c_int32),
        ("lastLapTimeMS", c_int32),
        ("worldPosition", vec3),
        ("isCarInPitline", c_int32),
        ("isCarInPit", c_int32),
        ("carLeaderboardPosition", c_int32),
        ("carRealTimeLeaderboardPosition", c_int32),
        ("spLineLength", c_float),
        ("isConnected", c_int32),
        ("suspensionDamage", c_float * 4),
        ("engineLifeLeft", c_float),
        ("tyreInflation", c_float * 4),
    ]


class SPageFileSimHub(ctypes.Structure):
    _pack_ = 4
    _fields_ = [
        ("numVehicles", c_int32),
        ("focusVehicle", c_int32),
        ("serverName", c_char * 512),
        ("vehicleInfo", acsVehicleInfo * 128),
        ("acInstallPath", c_char * 512),
        ("isInternalMemoryModuleLoaded", c_int32),
        ("pluginVersion", c_char * 32),
    ]


class SimHubShared:
    def __init__(self):
        self._acpmf_simhub = mmap.mmap(
            0, ctypes.sizeof(SPageFileSimHub), "acpmf_simhub_v2"
        )
        self.simhub = SPageFileSimHub.from_buffer(self._acpmf_simhub)

    def close(self):
        self._acpmf_simhub.close()

    def __del__(self):
        self.close()

    def getsharedmem(self):
        return self.simhub


class Simucube:
    def __init__(self):
        pygame.init()
        pygame.joystick.init()
        # Check if any joystick is connected
        joystick_count = pygame.joystick.get_count()
        if joystick_count == 0:
            print("No joystick/gamepad detected!")
            exit()
        simucube = None
        # Loop through all connected joysticks
        for i in range(joystick_count):
            joystick = pygame.joystick.Joystick(i)
            joystick.init()
            print(f"Joystick {i}: {joystick.get_name()}")
            print(f"  Axes: {joystick.get_numaxes()}")
            print(f"  Buttons: {joystick.get_numbuttons()}")
            print(f"  Hats: {joystick.get_numhats()}")
            print("-" * 40)
            if joystick.get_name() == "Simucube 2 Pro":
                simucube = i
        if simucube == None:
            exit
        self.joystick = pygame.joystick.Joystick(simucube)
        self.joystick.init()

    def getaxeradius(self):
        pygame.event.pump()
        return self.joystick.get_axis(0)


class Vjoy:
    def __init__(self):
        self.vjoy = pyvjoy.VJoyDevice(1)

    def float_to_vjoy_axis(value):
        value = max(-1.0, min(1.0, value))  # Clamp
        return int((value + 1) / 2 * 32768)

    def set_value(self, raw_output_value):
        output_value = Vjoy.float_to_vjoy_axis(raw_output_value)
        self.vjoy.set_axis(pyvjoy.HID_USAGE_X, output_value)


if __name__ == "__main__":
    shared = SimHubShared()
    simucube = Simucube()
    vjoy = Vjoy()
    try:
        while True:
            data = shared.getsharedmem()
            in_st = simucube.getaxeradius()
            print("Plugin version:", data.pluginVersion.decode().strip("\x00"))
            print("AC Install Path:", data.acInstallPath.decode().strip("\x00"))
            print("Server Name:", data.serverName.decode().strip("\x00"))
            print("Num Vehicles:", data.numVehicles)
            print("Focus Vehicle ID:", data.focusVehicle)

            for i in range(data.numVehicles):
                car = data.vehicleInfo[i]
                if car.isConnected:
                    ratio = 1
                    if car.speedMS < 40:
                        ratio = 4
                    elif car.speedMS > 150:
                        ratio = 1
                    else:
                        ratio = 2
                    out_st = ratio * in_st
                    if out_st < -1:
                        out_st = -1
                    elif out_st > 1:
                        out_st = 1
                    vjoy.set_value(out_st)
                    print(
                        f"  Speed (m/s): {car.speedMS:.2f} with Raw {in_st:.2f} => To {out_st:.2f}"
                    )
            print("------")
            time.sleep(0.1)
    except KeyboardInterrupt:
        print("Exiting...")
    finally:
        shared.close()

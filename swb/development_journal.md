# SWB: Developement Journal

## Goal

How to make a Simucube 2 Pro working as steering by wire in Assetto Corsa.
In another words, add a factor on angle input from Simucube to Assetto Corsa based on in game car telemetry.

## Developement

### Telemetry

To get in game car telemetry, we used shared memory provided by Assetto Corsa ([doc](https://docs.google.com/document/d/17TV3T75gN1GWg1W6hbFM1kZgYxNHfAvKBlE6AHqvGrw/edit?tab=t.0)).

- gas pedal
- brake pedal
- current fuel (liters)
- current gear
- rpm
- angle steer
- speed (Km/h)
- velocity (x,y,z)
- g-force (x,y,z)
- current force feedback

### Simucube 2 Pro

To get current Simucube 2 Pro, we used `pygame` and get device by name and read `x` axe value.

### Virtual Joystick

To set value to vjoy, we used `pyvjoy` and get device by name and pass value on `x` axe.

### Make Assetto Corsa working with Virtual Joystick

So our first idea, was to take `x` axe current value of Simucube 2 Pro with current car speed in Km/h and make a factor table to send into `x` axe Vjoy and to link Vjoy `x` axe as steering input axe.

```
Assetto Corsa car speed -\
                          |--> Python script --> Vjoy --> Assetto Corsa steer
Simucube 2 Pro ----------/
```

But with this we lost force feedback into Simucube 2 Pro because is not directly related to Assetto Corsa

### Tools

- Adjusts force feedback gain in real time: [Assetto-Corsa-FFBAntiClip](https://github.com/Damgam/Assetto-Corsa-FFBAntiClip)

# Telemetry Documentation

The handshake process to connect and receive Remote Telemetry data from Assetto Corsa via UDP works as follows: your app first must create an UDP socket and connect to a PC address running Assetto Corsa (ACServer).
The connection port number is `9996`

Your application must send a structured data using the following format:

```c++
struct handshaker{
    int identifier;
    int version;
    int operationId;
};
```

- `identifier`: [not used in the current Remote Telemetry version by AC]
- `version`: [not used in the current Remote Telemetry version by AC]
- `operationId`: This is the type of operation required by the client.
  The following operations are now available:
  - ```
    HANDSHAKE = 0           Client wants to start the communication
    SUBSCRIBE_UPDATE = 1    Client wants to be updated from the specific ACServer
    SUBSCRIBE_SPOT = 2      Client wants to be updated from the specific ACServer just for SPOT Events (e.g.: the end of a lap)
    DISMISS = 3             Client wants to leave the communication with ACServer
    ```

The first handshaking phase your application will need to send the following structured data to ACServer:

```c++
struct handshaker;
handshaker.identifier = 1 ;
handshaker.version = 1 ;
handshaker.operationId= 0 ;
```

Your application will receive the following struct as response:

```c++
struct handshackerResponse{
    char carName[50];
    char driverName[50];
    int identifier;
    int version;
    char trackName[50];
    char trackConfig[50];
};
```

- `carName[50]`: name of the car that the player is driving on the AC Server
- `drivername[50]`: name of the driver running on the AC Server
- `identifier`: for now is just 4242, this code will identify different status, as “NOT AVAILABLE” for connection
- `version`: for now is set to 1, this will identify the version running on the AC Server
- `trackName[50]`: name of the track on the AC Server
- `trackConfig[50]`: track configuration

Again the client must send the structured data `handshaker`.
Now operationId must be one of the following options:

```
SUBSCRIBE_UPDATE = 1    Client wants to be updated from the specific ACServer
SUBSCRIBE_SPOT = 2      Client wants to be updated from the specific ACServer just for SPOT Events (e.g.: the end of a lap).
```

After this phase the Client is added as a listener to AC Remote Telemetry listeners.

For each physics step, ACServer will call the update function to all the listeners.
If the client subscribed himself with SUBSCRIBE_UPDATE identifier, it will receive the following structured data:

```
struct RTCarInfo
{
    char identifier;
    int size;

    float speed_Kmh;
    float speed_Mph;
    float speed_Ms;

    bool isAbsEnabled;
    bool isAbsInAction;
    bool isTcInAction;
    bool isTcEnabled;
    bool isInPit;
    bool isEngineLimiterOn;

    float accG_vertical;
    float accG_horizontal;
    float accG_frontal;

    int lapTime;data.radius
        int lastLap;
    int bestLap;
    int lapCount;

    float gas;
    float brake;
    float clutch;
    float engineRPM;
    float steer;
    int gear;
    float cgHeight;

    float wheelAngularSpeed[4];
    float slipAngle[4];
    float slipAngle_ContactPatch[4];
    float slipRatio[4];
    float tyreSlip[4];
    float ndSlip[4];
    float load[4];
    float Dy[4];
    float Mz[4];
    float tyreDirtyLevel[4];
    float camberRAD[4];
    float tyreRadius[4];
    float tyreLoadedRadius[4];
    float suspensionHeight[4];
    float carPositionNormalized;
    float carSlope;
    float carCoordinates[3];
}
```

If the client subscribed himself with SUBSCRIBE_SPOT identifier, it will receive the following structured data whenever a spot event is triggered (for example for the end of a lap).
Differently from SUBSCRIBE_UPDATE, this event will interest all the cars in the AC session:

```
struct RTLap
{
    int carIdentifierNumber;
    int lap;
    char driverName[50];
    char carName[50];
    int time;
};
```

AA client to dismiss itself by sending the structured data `handshaker` with DISMISS = 3 as `operationId`.

AC client can receive the following commands through the UDP connection:

```
66      Current session gets closed
67      Camera is set in Cockpit mode
68      Restart current session
69      A new session gets started
70      Previous Car gets focused
71      Player Car gets focused
72      Next Car gets focused
73      Camera cycles between F6 cameras
74      Camera cycles between F3 cameras
75      Car gets teleported to pit
76      Car gets teleported to pit and Race Control screen is shown
77      Add an ID (Int value). Focus goes to the car with that ID (single player)
78      Add an ID (Int value). Focus goes to the car with that ID (multiplayer)
79      Add an ID (Int value). Car with that ID is returned to the pit
80      Stop the car gently
```

message type RT_GET_IDSESSION 5
return the own id session of multiplayer session (a simple int)

message type RT_GET_CARS 4
return a RTDriversInfoResponse struct:

```
struct RTDriversInfoResponse{
    unsigned int numPilots = 0;
    unsigned int numCars = 0;
};
```

followed by a list with numCars size of RTDriverInfo struct:

```
struct RTDriverInfo{
    wchar_t driverName[CHAR_LENGHT];
    wchar_t carName[CHAR_LENGHT];
    unsigned int idSession;
    unsigned int idLocal;
};
```

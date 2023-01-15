# TT Server
Server for Thunder Track

## TODO:



## API Structure

### /version
-> "Version ID

### /add/{id}/
->same thing back?
    -> if message is not returned keep sending

### /delete/{id}/
->same thing back
    -> if message is not returned keep sending

### /get
if no errors:
-> {nil}
else:
-> error json structure:

```json
{
    "1234": {
        "description" : "overcurrent",
        "time": 12134,
        "urgency": 0,            
        "area": "BMS"
    },
    "12345": {
        "description": "overheat",
        "time": 121345,
        "urgency": 10,
        "area": "Motor"
    }
}
```

For time, maybe server takes when server gets the error, seconds since start of day UTC?

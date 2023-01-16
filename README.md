# TT Server
Server for Thunder Track  
Most information about Thunder Track (APIs, return structures etc.) will be stored here
  
## TODO:
  
## API Structure

### /version
-> "Version ID"   

### /add/{id}/
if the code is in the dictionary  -> return number
else -> return "Invalud

### /remove/{id}/
if it was an error -> return number
else -> return "Invalid"

### /get
if no errors:
-> {} (in Ruby this returns nil)
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
The time is UNIX epoch (seconds) (UTC)
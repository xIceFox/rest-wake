# Rest-Wake Backend
This is the backend part of the project, which provides the REST-API with endpoints to wake devices in the local network.

## How-to-use:
### Wake device/s by mac address:
This endpoint allows to wake devices by mac address.

``POST /api/wake/mac/{mac_address}`` Wakes a single device, with the mac address provided.

``POST /api/wake/mac`` Wakes multiple devices, with the mac addresses provided by the JSON-Array in the request body.\
Example json body: `` ["AA:BB:CC:DD:EE:FF", "AB:CD:EF:10:20:30"] ``

### Device management:
The device management endpoint allows to create, get, update and delete devices in the local database, to provide a quicker way to wake devices by name instead of the hard to remember mac address.

**Device json example**: 
```json
{
  "name": "SelfChosenDeviceName", 
  "mac": "AA:BB:CC:DD:EE:FF"
}
```

``GET /api/device/{name}`` Get the specified device by the name provided in the route.

``POST /api/device`` Creates the device in the local database with the user provided json object in body.

``PUT /api/device`` Updates the mac address of the device specified in the json body. The provided device needs to match to a device saved in the database.

``DELETE /api/device/{name}`` Deletes the specified device by the name provided in the route.

``GET /api/devices?page={page_index}`` Gets all devices from the database, paginated to 50 devices per request. The page index can be specified in the query parameters.

### Wake device by name:
This endpoint allows to wake devices by the name in the local database. To create a device, see the "Device management" endpoint above.

``POST /api/wake/device/{name}`` Wakes the specified device by the device name provided in the route.

``POST /api/wake/devices`` Wakes all the specified devices by the names provided in the json body.
Example json body: ``["Server01", "Test-PC"]``

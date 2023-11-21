## Rest-Wake
This project aims to provide a possibility, to easily wake up devices via the wake-on-lan protocol. 
Often, devices need to wake up over a vpn connection, which is not straightforward, due to the wake-on-lan-packet not being transported by the vpn. This results from the fact that vpn protocols often transport packets created at layer 3 in the network stack and wake-on-lan packets functioning at layer 2.
To overcome this issue, this application can be hosted in the remote network, to send local wake-on-lan packets, triggered by REST-API (layer 7) calls over the vpn-connection.

The project is seperated into a backend (REST-API) and an optional frontend, which is developed in the future to provide more user-friendliness.

Due to the project being relatively new, the basic functionality has only been implemented into the backend. REST-API Endpoints are not final yet, so they can change over time.

For REST-API Documentation, see [backend/README.md](backend/README.md)

Every new contributor is welcome! :)

## Example Requests
`$ curl localhost:8082`
_{"server_id":0,"request_count":9,"message":[]}_

`$ curl -X POST -H "Content-Type: application/json" -d '{"message": "helllo"}' localhost:8082/send`
_{"server_id":1,"request_count":1,"message":"hello"}_

`$ curl -X POST -H "Content-Type: application/json" -d '{"message": "helllo2"}' localhost:8082/send`
_{"server_id":1,"request_count":1,"message":"hello2"}_

`$ curl localhost:8082`
_{"server_id":1,"request_count":3,"message":["helllo","helllo2"]}_

`$ curl -X POST localhost:8082/clear`
_{"server_id":4,"request_count":1,"message":[]}_

`$ curl localhost:8082`
_{"server_id":1,"request_count":3,"message":[]}_

`$ curl -X POST -H "Content-Type: application/json" -d '{"bad": "hello"}' localhost:8082/send`

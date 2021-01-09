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


## Lookup example requests
`$ curl localhost:8082`
_{"server_id":1,"request_count":3,"message":[]}_

`$ curl localhost:8082/lookup/2`
_{"server_id":0,"request_count":3,"result":null}_

`$ curl -X POST -H "Content-Type: application/json" -d '{"message": "helllo"}' localhost:8082/send`
_{"server_id":1,"request_count":1,"message":"hello"}_

`$ curl -X POST -H "Content-Type: application/json" -d '{"message": "helllo again"}' localhost:8082/send`
_{"server_id":1,"request_count":1,"message":"hello again"}_

`$ curl -X POST -H "Content-Type: application/json" -d '{"message": "goodbye"}' localhost:8082/send`
_{"server_id":1,"request_count":1,"message":"goodbye"}_

`$ curl localhost:8082`
_{"server_id":1,"request_count":1,"message":["helllo","helllo again","goodbye"]}_

`$ curl localhost:8082/lookup/0`
_{"server_id":2,"request_count":1,"result":"helllo"}_

`$ curl localhost:8082/lookup/1`
_{"server_id":3,"request_count":1,"result":"helllo again"}_

`$ curl localhost:8082/lookup/2`
_{"server_id":5,"request_count":1,"result":"goodbye"}_

`$ curl localhost:8082/lookup/foo`
_can not parse "foo" to a u64_

`$ curl localhost:8082/lookup/99`
_{"server_id":4,"request_count":2,"result":null}_
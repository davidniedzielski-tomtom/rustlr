python -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/openlr_services.proto
protoc -I../protos --pyi_out=. ../protos/openlr_services.proto


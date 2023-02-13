import logging
import grpc
import argparse

from concurrent import futures
import openlr_services_pb2_grpc
from shapely.geometry import Point, LineString
from shapely.ops import nearest_points
from shapely.wkt import loads
from openlr_services_pb2 import BinaryDecodeRequest, BinaryDecodeResponse, LoggingLevel

def generate_decoding_request(code, agent, i):
    return BinaryDecodeRequest(
            id=i,
            code=code,
            agent_name=agent, 
            agent_protocol="grpc",
            decoding_parameter_set="default",
            logging_level=5
    )

def stream_decoding_requests(code, agent, count):
    msgs = [generate_decoding_request(code, agent, i) for i in range(count)]
    for msg in msgs:
        yield(msg)
    
def run(server, code, agent, count):
    with grpc.insecure_channel(server) as channel:
        stub = openlr_services_pb2_grpc.DecoderStub(channel)
        responses = stub.DecodeBinary(stream_decoding_requests(code, agent, count)) 
        for response in responses:
            print(response)
    

if __name__ == '__main__':
    logging.basicConfig()
    parser = argparse.ArgumentParser()
    parser.add_argument("-a", "--agent", help="URL of map agent", required=True, type=str)
    parser.add_argument("-s", "--server", help="URL of openlr server", required=True, type=str)
    parser.add_argument("-n", "--count", help="Number of requests to stream", nargs='?', const=5, type=int, default=5)
    parser.add_argument("-c", "--code", help="OpenLR code", required=True, type=str)
    args = parser.parse_args()
    run(args.server, args.code, args.agent, args.count)

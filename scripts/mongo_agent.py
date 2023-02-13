import argparse
from pymongo import MongoClient
import logging
import grpc

from concurrent import futures
import openlr_services_pb2_grpc
from pyproj import Geod
from shapely.geometry import Point, LineString
from shapely.ops import nearest_points
from shapely.wkt import loads
from openlr_services_pb2 import EdgeSet, Edge, Coordinate, NearbyEdgesResponse

class MapAgentServicer(openlr_services_pb2_grpc.MapAgentServicer):
    def __init__(self, mongo_address):
        # Connect to the MongoDB instance
        client = MongoClient(mongo_address)
        self.db = client.aus_mn
        
    def make_edge(self, e):
        c = e["geometry"]["coordinates"]
        return Edge(
            id     = e["properties"]["id"],
            meta   = e["properties"]["meta"],
            fow    = e["properties"]["fow"],
            frc    = e["properties"]["frc"],
            len    = round(e["properties"]["len"]),
            coords = [Coordinate(longitude=p[0], latitude=p[1])
                    for p in c]
        )
    
    def make_edge_reversed(self, e):
        c = e["geometry"]["coordinates"]
        c.reverse()
        return Edge(
            id     = -e["properties"]["id"],
            meta   = e["properties"]["meta"],
            fow    = e["properties"]["fow"],
            frc    = e["properties"]["frc"],
            len    = round(e["properties"]["len"]),
            coords = [Coordinate(longitude=p[0], latitude=p[1])
                    for p in c]
        )
    
    def find_nearby(self, lon, lat, radius):
        res = []
        roads = self.db.roads.find( 
            {"geometry": {"$near": {"$geometry": {"type": "Point", "coordinates": [lon, lat]}, "$maxDistance": radius }}}
        )
    
        for road in roads:
            if road["properties"]["flowdir"] == 2:
                res.append(self.make_edge_reversed(road))
            else:
                res.append(self.make_edge(road))
                
            # if the road is two-way, also add the reversed edge
            if road["properties"]["flowdir"] == 1:
                res.append(self.make_edge_reversed(road))
        
        return res

    def GetNearbyEdges(self, request, context):
        res = []
        for p in request.points:
            res.append(EdgeSet(edges=self.find_nearby(
                float(p.longitude), float(p.latitude), float(request.radius))))
        print(res)
        return NearbyEdgesResponse(edge_sets=res)

    def GetNextEdges(self, request, context):
        res = []
        # Find the road whose "id" and "meta" fields match the parameters
        
        road = self.db.roads.find_one({"properties.id": abs(request.id), "properties.meta": request.meta})
    
        # If the road was found, search for its successors
        if road:
            # determine the successors' entry intersection based on the source edge's id
            src_int = road["properties"]["to_int"] if request.id >= 0 else road["properties"]["from_int"]
            successors = self.db.roads.find( 
                { "$or": [ 
                    { "$and": [ 
                        { "properties.from_int": src_int }, 
                        { "properties.flowdir": { "$ne": 2 } }] }, 
                    { "$and": [ 
                        { "properties.to_int": src_int }, 
                        { "properties.flowdir": 2 }] 
                }]})
            
            # examine each potential successor
            for succ in successors:
                # Ignore the src edge's peer
                if abs(request.id) != succ["properties"]["id"]:
                    # Reverse the successor of needed 
                    if succ["properties"]["to_int"] == src_int:
                        res.append(self.make_edge_reversed(succ))
                    else:
                        res.append(self.make_edge(succ))
        return EdgeSet(edges=res)
        

def serve(address, mongo_address):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    openlr_services_pb2_grpc.add_MapAgentServicer_to_server(
        MapAgentServicer(mongo_address), server)
    server.add_insecure_port(address)
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    parser = argparse.ArgumentParser()
    parser.add_argument("-m", "--mongodb", help="MongoDB URL", nargs='?', const="127.0.0.1:27017", type=str, default="127.0.0.1:27017")
    parser.add_argument("-g", "--grpc", help="gRPC listening URL", nargs='?', const="[::]:8083", type=str, default="[::]:8083")
    args = parser.parse_args()

    grpc_address = '[::]:8083'
    mongo_address= "127.0.0.1:27017"
    serve(args.grpc, args.mongodb)

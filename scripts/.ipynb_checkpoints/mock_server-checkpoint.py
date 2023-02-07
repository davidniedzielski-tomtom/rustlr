import logging
import grpc

from concurrent import futures
import openlr_services_pb2_grpc
from pyproj import Geod
from shapely.geometry import Point, LineString
from shapely.ops import nearest_points
from shapely.wkt import loads
from openlr_services_pb2 import EdgeSet, Edge, Coordinate, NearbyEdgesResponse


class MapServiceServicer(openlr_services_pb2_grpc.MapServiceServicer):
    def __init__(self, fn):
        self.db = []
        self.edge_map = {}
        with open(fn) as f:
            for line in f.readlines():
                fields = line.split(":")
                rec = {
                    "id": int(fields[0]),
                    "meta": fields[1].replace("\"", ""),
                    "fow": int(fields[2]),
                    "frc": int(fields[3]),
                    "from_int": int(fields[5]),
                    "to_int": int(fields[6]),
                    "len": int(fields[7]),
                    "geom": loads(fields[8].replace("\"", ""))
                }
                self.db.append(rec)
                self.edge_map[rec["id"]] = rec

    def make_edge(self, e):
        return Edge(
            id=e["id"],
            meta=e["meta"],
            fow=e["fow"],
            frc=e["frc"],
            len=e["len"],
            coords=[Coordinate(longitude=p[0], latitude=p[1])
                    for p in e["geom"].coords]
        )

    def find_nearby(self, lon, lat, radius):
        point = Point(lon, lat)
        res = []
        geod = Geod(ellps="WGS84")
        for e in self.db:
            distance = geod.geometry_length(
                LineString(nearest_points(e["geom"], point)))
            if (distance <= radius):
                res.append(self.make_edge(e))
        return res

    def GetNearbyEdges(self, request, context):
        res = []
        for p in request.points:
            res.append(EdgeSet(edges=self.find_nearby(
                float(p.longitude), float(p.latitude), float(request.radius))))
        return NearbyEdgesResponse(edge_sets=res)

    def GetNextEdges(self, request, context):
        res = []
        src = self.edge_map[request.id]
        return EdgeSet(edges=[self.make_edge(e) for e in self.db if e["from_int"] == src["to_int"] and e["to_int"] != src["from_int"]])


def serve(address, fn):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    openlr_services_pb2_grpc.add_MapServiceServicer_to_server(
        MapServiceServicer(fn), server)
    server.add_insecure_port(address)
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    logging.basicConfig()
    fn = '/Users/dave/projects/rust/rustlr/openlr/test_data/test1.csv'
    address = '[::]:8083'
    serve(address, fn)

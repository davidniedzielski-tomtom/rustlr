# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

import openlr_services_pb2 as openlr__services__pb2


class MapServiceStub(object):
  # missing associated documentation comment in .proto file
  pass

  def __init__(self, channel):
    """Constructor.

    Args:
      channel: A grpc.Channel.
    """
    self.RadiusSearch = channel.unary_unary(
        '/openlr_services.MapService/RadiusSearch',
        request_serializer=openlr__services__pb2.RadiusSearchRequest.SerializeToString,
        response_deserializer=openlr__services__pb2.RadiusSearchResponse.FromString,
        )
    self.NextEdges = channel.unary_unary(
        '/openlr_services.MapService/NextEdges',
        request_serializer=openlr__services__pb2.NextEdgesRequest.SerializeToString,
        response_deserializer=openlr__services__pb2.EdgeSet.FromString,
        )


class MapServiceServicer(object):
  # missing associated documentation comment in .proto file
  pass

  def RadiusSearch(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')

  def NextEdges(self, request, context):
    # missing associated documentation comment in .proto file
    pass
    context.set_code(grpc.StatusCode.UNIMPLEMENTED)
    context.set_details('Method not implemented!')
    raise NotImplementedError('Method not implemented!')


def add_MapServiceServicer_to_server(servicer, server):
  rpc_method_handlers = {
      'RadiusSearch': grpc.unary_unary_rpc_method_handler(
          servicer.RadiusSearch,
          request_deserializer=openlr__services__pb2.RadiusSearchRequest.FromString,
          response_serializer=openlr__services__pb2.RadiusSearchResponse.SerializeToString,
      ),
      'NextEdges': grpc.unary_unary_rpc_method_handler(
          servicer.NextEdges,
          request_deserializer=openlr__services__pb2.NextEdgesRequest.FromString,
          response_serializer=openlr__services__pb2.EdgeSet.SerializeToString,
      ),
  }
  generic_handler = grpc.method_handlers_generic_handler(
      'openlr_services.MapService', rpc_method_handlers)
  server.add_generic_rpc_handlers((generic_handler,))

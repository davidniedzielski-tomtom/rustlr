# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: openlr_services.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='openlr_services.proto',
  package='openlr_services',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=_b('\n\x15openlr_services.proto\x12\x0fopenlr_services\"1\n\nCoordinate\x12\x11\n\tlongitude\x18\x01 \x01(\x01\x12\x10\n\x08latitude\x18\x02 \x01(\x01\"\x92\x03\n\x04\x45\x64ge\x12\n\n\x02id\x18\x01 \x01(\x03\x12\x0c\n\x04meta\x18\x02 \x01(\t\x12&\n\x03\x66ow\x18\x03 \x01(\x0e\x32\x19.openlr_services.Edge.FOW\x12&\n\x03\x66rc\x18\x04 \x01(\x0e\x32\x19.openlr_services.Edge.FRC\x12\x0b\n\x03len\x18\x05 \x01(\r\x12+\n\x06\x63oords\x18\x06 \x03(\x0b\x32\x1b.openlr_services.Coordinate\"\x8e\x01\n\x03\x46OW\x12\r\n\tUNDEFINED\x10\x00\x12\x0c\n\x08MOTORWAY\x10\x01\x12\x17\n\x13MULTIPLECARRIAGEWAY\x10\x02\x12\x15\n\x11SINGLECARRIAGEWAY\x10\x03\x12\x0e\n\nROUNDABOUT\x10\x04\x12\x11\n\rTRAFFICSQUARE\x10\x05\x12\x0c\n\x08SLIPROAD\x10\x06\x12\t\n\x05OTHER\x10\x07\"U\n\x03\x46RC\x12\x08\n\x04\x46RC0\x10\x00\x12\x08\n\x04\x46RC1\x10\x01\x12\x08\n\x04\x46RC2\x10\x02\x12\x08\n\x04\x46RC3\x10\x03\x12\x08\n\x04\x46RC4\x10\x04\x12\x08\n\x04\x46RC5\x10\x05\x12\x08\n\x04\x46RC6\x10\x06\x12\x08\n\x04\x46RC7\x10\x07\"/\n\x07\x45\x64geSet\x12$\n\x05\x65\x64ges\x18\x01 \x03(\x0b\x32\x15.openlr_services.Edge\"R\n\x13RadiusSearchRequest\x12+\n\x06points\x18\x01 \x03(\x0b\x32\x1b.openlr_services.Coordinate\x12\x0e\n\x06radius\x18\x02 \x01(\r\"C\n\x14RadiusSearchResponse\x12+\n\tedge_sets\x18\x01 \x03(\x0b\x32\x18.openlr_services.EdgeSet\",\n\x10NextEdgesRequest\x12\n\n\x02id\x18\x01 \x01(\x03\x12\x0c\n\x04meta\x18\x02 \x01(\t2\xb3\x01\n\nMapService\x12[\n\x0cRadiusSearch\x12$.openlr_services.RadiusSearchRequest\x1a%.openlr_services.RadiusSearchResponse\x12H\n\tNextEdges\x12!.openlr_services.NextEdgesRequest\x1a\x18.openlr_services.EdgeSetb\x06proto3')
)



_EDGE_FOW = _descriptor.EnumDescriptor(
  name='FOW',
  full_name='openlr_services.Edge.FOW',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='UNDEFINED', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='MOTORWAY', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='MULTIPLECARRIAGEWAY', index=2, number=2,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='SINGLECARRIAGEWAY', index=3, number=3,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='ROUNDABOUT', index=4, number=4,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='TRAFFICSQUARE', index=5, number=5,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='SLIPROAD', index=6, number=6,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='OTHER', index=7, number=7,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=267,
  serialized_end=409,
)
_sym_db.RegisterEnumDescriptor(_EDGE_FOW)

_EDGE_FRC = _descriptor.EnumDescriptor(
  name='FRC',
  full_name='openlr_services.Edge.FRC',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='FRC0', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC1', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC2', index=2, number=2,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC3', index=3, number=3,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC4', index=4, number=4,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC5', index=5, number=5,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC6', index=6, number=6,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FRC7', index=7, number=7,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=411,
  serialized_end=496,
)
_sym_db.RegisterEnumDescriptor(_EDGE_FRC)


_COORDINATE = _descriptor.Descriptor(
  name='Coordinate',
  full_name='openlr_services.Coordinate',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='longitude', full_name='openlr_services.Coordinate.longitude', index=0,
      number=1, type=1, cpp_type=5, label=1,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='latitude', full_name='openlr_services.Coordinate.latitude', index=1,
      number=2, type=1, cpp_type=5, label=1,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=42,
  serialized_end=91,
)


_EDGE = _descriptor.Descriptor(
  name='Edge',
  full_name='openlr_services.Edge',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='openlr_services.Edge.id', index=0,
      number=1, type=3, cpp_type=2, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='meta', full_name='openlr_services.Edge.meta', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='fow', full_name='openlr_services.Edge.fow', index=2,
      number=3, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='frc', full_name='openlr_services.Edge.frc', index=3,
      number=4, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='len', full_name='openlr_services.Edge.len', index=4,
      number=5, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='coords', full_name='openlr_services.Edge.coords', index=5,
      number=6, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _EDGE_FOW,
    _EDGE_FRC,
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=94,
  serialized_end=496,
)


_EDGESET = _descriptor.Descriptor(
  name='EdgeSet',
  full_name='openlr_services.EdgeSet',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='edges', full_name='openlr_services.EdgeSet.edges', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=498,
  serialized_end=545,
)


_RADIUSSEARCHREQUEST = _descriptor.Descriptor(
  name='RadiusSearchRequest',
  full_name='openlr_services.RadiusSearchRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='points', full_name='openlr_services.RadiusSearchRequest.points', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='radius', full_name='openlr_services.RadiusSearchRequest.radius', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=547,
  serialized_end=629,
)


_RADIUSSEARCHRESPONSE = _descriptor.Descriptor(
  name='RadiusSearchResponse',
  full_name='openlr_services.RadiusSearchResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='edge_sets', full_name='openlr_services.RadiusSearchResponse.edge_sets', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=631,
  serialized_end=698,
)


_NEXTEDGESREQUEST = _descriptor.Descriptor(
  name='NextEdgesRequest',
  full_name='openlr_services.NextEdgesRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='openlr_services.NextEdgesRequest.id', index=0,
      number=1, type=3, cpp_type=2, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='meta', full_name='openlr_services.NextEdgesRequest.meta', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=700,
  serialized_end=744,
)

_EDGE.fields_by_name['fow'].enum_type = _EDGE_FOW
_EDGE.fields_by_name['frc'].enum_type = _EDGE_FRC
_EDGE.fields_by_name['coords'].message_type = _COORDINATE
_EDGE_FOW.containing_type = _EDGE
_EDGE_FRC.containing_type = _EDGE
_EDGESET.fields_by_name['edges'].message_type = _EDGE
_RADIUSSEARCHREQUEST.fields_by_name['points'].message_type = _COORDINATE
_RADIUSSEARCHRESPONSE.fields_by_name['edge_sets'].message_type = _EDGESET
DESCRIPTOR.message_types_by_name['Coordinate'] = _COORDINATE
DESCRIPTOR.message_types_by_name['Edge'] = _EDGE
DESCRIPTOR.message_types_by_name['EdgeSet'] = _EDGESET
DESCRIPTOR.message_types_by_name['RadiusSearchRequest'] = _RADIUSSEARCHREQUEST
DESCRIPTOR.message_types_by_name['RadiusSearchResponse'] = _RADIUSSEARCHRESPONSE
DESCRIPTOR.message_types_by_name['NextEdgesRequest'] = _NEXTEDGESREQUEST
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Coordinate = _reflection.GeneratedProtocolMessageType('Coordinate', (_message.Message,), dict(
  DESCRIPTOR = _COORDINATE,
  __module__ = 'openlr_services_pb2'
  # @@protoc_insertion_point(class_scope:openlr_services.Coordinate)
  ))
_sym_db.RegisterMessage(Coordinate)

Edge = _reflection.GeneratedProtocolMessageType('Edge', (_message.Message,), dict(
  DESCRIPTOR = _EDGE,
  __module__ = 'openlr_services_pb2'
  # @@protoc_insertion_point(class_scope:openlr_services.Edge)
  ))
_sym_db.RegisterMessage(Edge)

EdgeSet = _reflection.GeneratedProtocolMessageType('EdgeSet', (_message.Message,), dict(
  DESCRIPTOR = _EDGESET,
  __module__ = 'openlr_services_pb2'
  # @@protoc_insertion_point(class_scope:openlr_services.EdgeSet)
  ))
_sym_db.RegisterMessage(EdgeSet)

RadiusSearchRequest = _reflection.GeneratedProtocolMessageType('RadiusSearchRequest', (_message.Message,), dict(
  DESCRIPTOR = _RADIUSSEARCHREQUEST,
  __module__ = 'openlr_services_pb2'
  # @@protoc_insertion_point(class_scope:openlr_services.RadiusSearchRequest)
  ))
_sym_db.RegisterMessage(RadiusSearchRequest)

RadiusSearchResponse = _reflection.GeneratedProtocolMessageType('RadiusSearchResponse', (_message.Message,), dict(
  DESCRIPTOR = _RADIUSSEARCHRESPONSE,
  __module__ = 'openlr_services_pb2'
  # @@protoc_insertion_point(class_scope:openlr_services.RadiusSearchResponse)
  ))
_sym_db.RegisterMessage(RadiusSearchResponse)

NextEdgesRequest = _reflection.GeneratedProtocolMessageType('NextEdgesRequest', (_message.Message,), dict(
  DESCRIPTOR = _NEXTEDGESREQUEST,
  __module__ = 'openlr_services_pb2'
  # @@protoc_insertion_point(class_scope:openlr_services.NextEdgesRequest)
  ))
_sym_db.RegisterMessage(NextEdgesRequest)



_MAPSERVICE = _descriptor.ServiceDescriptor(
  name='MapService',
  full_name='openlr_services.MapService',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  serialized_start=747,
  serialized_end=926,
  methods=[
  _descriptor.MethodDescriptor(
    name='RadiusSearch',
    full_name='openlr_services.MapService.RadiusSearch',
    index=0,
    containing_service=None,
    input_type=_RADIUSSEARCHREQUEST,
    output_type=_RADIUSSEARCHRESPONSE,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='NextEdges',
    full_name='openlr_services.MapService.NextEdges',
    index=1,
    containing_service=None,
    input_type=_NEXTEDGESREQUEST,
    output_type=_EDGESET,
    serialized_options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_MAPSERVICE)

DESCRIPTOR.services_by_name['MapService'] = _MAPSERVICE

# @@protoc_insertion_point(module_scope)

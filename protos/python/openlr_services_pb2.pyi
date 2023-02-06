from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Coordinate(_message.Message):
    __slots__ = ["latitude", "longitude"]
    LATITUDE_FIELD_NUMBER: _ClassVar[int]
    LONGITUDE_FIELD_NUMBER: _ClassVar[int]
    latitude: float
    longitude: float
    def __init__(self, longitude: _Optional[float] = ..., latitude: _Optional[float] = ...) -> None: ...

class Edge(_message.Message):
    __slots__ = ["coords", "fow", "frc", "id", "len", "meta"]
    class FOW(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    class FRC(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    COORDS_FIELD_NUMBER: _ClassVar[int]
    FOW_FIELD_NUMBER: _ClassVar[int]
    FRC0: Edge.FRC
    FRC1: Edge.FRC
    FRC2: Edge.FRC
    FRC3: Edge.FRC
    FRC4: Edge.FRC
    FRC5: Edge.FRC
    FRC6: Edge.FRC
    FRC7: Edge.FRC
    FRC_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    LEN_FIELD_NUMBER: _ClassVar[int]
    META_FIELD_NUMBER: _ClassVar[int]
    MOTORWAY: Edge.FOW
    MULTIPLECARRIAGEWAY: Edge.FOW
    OTHER: Edge.FOW
    ROUNDABOUT: Edge.FOW
    SINGLECARRIAGEWAY: Edge.FOW
    SLIPROAD: Edge.FOW
    TRAFFICSQUARE: Edge.FOW
    UNDEFINED: Edge.FOW
    coords: _containers.RepeatedCompositeFieldContainer[Coordinate]
    fow: Edge.FOW
    frc: Edge.FRC
    id: int
    len: int
    meta: str
    def __init__(self, id: _Optional[int] = ..., meta: _Optional[str] = ..., fow: _Optional[_Union[Edge.FOW, str]] = ..., frc: _Optional[_Union[Edge.FRC, str]] = ..., len: _Optional[int] = ..., coords: _Optional[_Iterable[_Union[Coordinate, _Mapping]]] = ...) -> None: ...

class EdgeSet(_message.Message):
    __slots__ = ["edges"]
    EDGES_FIELD_NUMBER: _ClassVar[int]
    edges: _containers.RepeatedCompositeFieldContainer[Edge]
    def __init__(self, edges: _Optional[_Iterable[_Union[Edge, _Mapping]]] = ...) -> None: ...

class NextEdgesRequest(_message.Message):
    __slots__ = ["id", "meta"]
    ID_FIELD_NUMBER: _ClassVar[int]
    META_FIELD_NUMBER: _ClassVar[int]
    id: int
    meta: str
    def __init__(self, id: _Optional[int] = ..., meta: _Optional[str] = ...) -> None: ...

class RadiusSearchRequest(_message.Message):
    __slots__ = ["points", "radius"]
    POINTS_FIELD_NUMBER: _ClassVar[int]
    RADIUS_FIELD_NUMBER: _ClassVar[int]
    points: _containers.RepeatedCompositeFieldContainer[Coordinate]
    radius: int
    def __init__(self, points: _Optional[_Iterable[_Union[Coordinate, _Mapping]]] = ..., radius: _Optional[int] = ...) -> None: ...

class RadiusSearchResponse(_message.Message):
    __slots__ = ["edge_sets"]
    EDGE_SETS_FIELD_NUMBER: _ClassVar[int]
    edge_sets: _containers.RepeatedCompositeFieldContainer[EdgeSet]
    def __init__(self, edge_sets: _Optional[_Iterable[_Union[EdgeSet, _Mapping]]] = ...) -> None: ...

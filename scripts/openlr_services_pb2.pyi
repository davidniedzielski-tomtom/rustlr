from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

BOTH_DIRECTIONS: Orientation
BOTH_SIDES: SideOfRoad
DEBUG: LoggingLevel
DESCRIPTOR: _descriptor.FileDescriptor
ERROR: LoggingLevel
FATAL: LoggingLevel
FIRST_TO_SECOND: Orientation
INFO: LoggingLevel
LEFT_SIDE: SideOfRoad
NO_OR_UNKNOWN: Orientation
ON_OR_NOT_APPLICABLE: SideOfRoad
RIGHT_SIDE: SideOfRoad
SECOND_TO_FIRST: Orientation
TRACE: LoggingLevel
WARN: LoggingLevel

class BinaryDecodeRequest(_message.Message):
    __slots__ = ["agent_name", "agent_protocol", "code", "decoding_parameter_set", "id", "logging_level"]
    AGENT_NAME_FIELD_NUMBER: _ClassVar[int]
    AGENT_PROTOCOL_FIELD_NUMBER: _ClassVar[int]
    CODE_FIELD_NUMBER: _ClassVar[int]
    DECODING_PARAMETER_SET_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    LOGGING_LEVEL_FIELD_NUMBER: _ClassVar[int]
    agent_name: str
    agent_protocol: str
    code: str
    decoding_parameter_set: str
    id: int
    logging_level: LoggingLevel
    def __init__(self, id: _Optional[int] = ..., code: _Optional[str] = ..., decoding_parameter_set: _Optional[str] = ..., agent_protocol: _Optional[str] = ..., agent_name: _Optional[str] = ..., logging_level: _Optional[_Union[LoggingLevel, str]] = ...) -> None: ...

class BinaryDecodeResponse(_message.Message):
    __slots__ = ["decodeError", "elapsed_nanosecs", "elapsed_secs", "id", "lineLocation", "log", "pointAlongLineLocation"]
    DECODEERROR_FIELD_NUMBER: _ClassVar[int]
    ELAPSED_NANOSECS_FIELD_NUMBER: _ClassVar[int]
    ELAPSED_SECS_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    LINELOCATION_FIELD_NUMBER: _ClassVar[int]
    LOG_FIELD_NUMBER: _ClassVar[int]
    POINTALONGLINELOCATION_FIELD_NUMBER: _ClassVar[int]
    decodeError: DecodeError
    elapsed_nanosecs: int
    elapsed_secs: int
    id: int
    lineLocation: LineLocation
    log: _containers.RepeatedCompositeFieldContainer[LogMessage]
    pointAlongLineLocation: PointAlongLineLocation
    def __init__(self, id: _Optional[int] = ..., elapsed_secs: _Optional[int] = ..., elapsed_nanosecs: _Optional[int] = ..., log: _Optional[_Iterable[_Union[LogMessage, _Mapping]]] = ..., decodeError: _Optional[_Union[DecodeError, _Mapping]] = ..., lineLocation: _Optional[_Union[LineLocation, _Mapping]] = ..., pointAlongLineLocation: _Optional[_Union[PointAlongLineLocation, _Mapping]] = ...) -> None: ...

class Coordinate(_message.Message):
    __slots__ = ["latitude", "longitude"]
    LATITUDE_FIELD_NUMBER: _ClassVar[int]
    LONGITUDE_FIELD_NUMBER: _ClassVar[int]
    latitude: float
    longitude: float
    def __init__(self, longitude: _Optional[float] = ..., latitude: _Optional[float] = ...) -> None: ...

class DecodeError(_message.Message):
    __slots__ = ["reason"]
    REASON_FIELD_NUMBER: _ClassVar[int]
    reason: str
    def __init__(self, reason: _Optional[str] = ...) -> None: ...

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

class LineLocation(_message.Message):
    __slots__ = ["edge", "neg_off", "pos_off"]
    EDGE_FIELD_NUMBER: _ClassVar[int]
    NEG_OFF_FIELD_NUMBER: _ClassVar[int]
    POS_OFF_FIELD_NUMBER: _ClassVar[int]
    edge: _containers.RepeatedCompositeFieldContainer[Edge]
    neg_off: OffsetRange
    pos_off: OffsetRange
    def __init__(self, edge: _Optional[_Iterable[_Union[Edge, _Mapping]]] = ..., pos_off: _Optional[_Union[OffsetRange, _Mapping]] = ..., neg_off: _Optional[_Union[OffsetRange, _Mapping]] = ...) -> None: ...

class LogMessage(_message.Message):
    __slots__ = ["level", "msg"]
    LEVEL_FIELD_NUMBER: _ClassVar[int]
    MSG_FIELD_NUMBER: _ClassVar[int]
    level: LoggingLevel
    msg: str
    def __init__(self, level: _Optional[_Union[LoggingLevel, str]] = ..., msg: _Optional[str] = ...) -> None: ...

class NearbyEdgesRequest(_message.Message):
    __slots__ = ["points", "radius"]
    POINTS_FIELD_NUMBER: _ClassVar[int]
    RADIUS_FIELD_NUMBER: _ClassVar[int]
    points: _containers.RepeatedCompositeFieldContainer[Coordinate]
    radius: int
    def __init__(self, points: _Optional[_Iterable[_Union[Coordinate, _Mapping]]] = ..., radius: _Optional[int] = ...) -> None: ...

class NearbyEdgesResponse(_message.Message):
    __slots__ = ["edge_sets"]
    EDGE_SETS_FIELD_NUMBER: _ClassVar[int]
    edge_sets: _containers.RepeatedCompositeFieldContainer[EdgeSet]
    def __init__(self, edge_sets: _Optional[_Iterable[_Union[EdgeSet, _Mapping]]] = ...) -> None: ...

class NextEdgesRequest(_message.Message):
    __slots__ = ["id", "meta"]
    ID_FIELD_NUMBER: _ClassVar[int]
    META_FIELD_NUMBER: _ClassVar[int]
    id: int
    meta: str
    def __init__(self, id: _Optional[int] = ..., meta: _Optional[str] = ...) -> None: ...

class OffsetRange(_message.Message):
    __slots__ = ["lb", "ub"]
    LB_FIELD_NUMBER: _ClassVar[int]
    UB_FIELD_NUMBER: _ClassVar[int]
    lb: int
    ub: int
    def __init__(self, lb: _Optional[int] = ..., ub: _Optional[int] = ...) -> None: ...

class PointAlongLineLocation(_message.Message):
    __slots__ = ["edge", "orientation", "pos_off", "side_of_road"]
    EDGE_FIELD_NUMBER: _ClassVar[int]
    ORIENTATION_FIELD_NUMBER: _ClassVar[int]
    POS_OFF_FIELD_NUMBER: _ClassVar[int]
    SIDE_OF_ROAD_FIELD_NUMBER: _ClassVar[int]
    edge: _containers.RepeatedCompositeFieldContainer[Edge]
    orientation: Orientation
    pos_off: OffsetRange
    side_of_road: SideOfRoad
    def __init__(self, edge: _Optional[_Iterable[_Union[Edge, _Mapping]]] = ..., pos_off: _Optional[_Union[OffsetRange, _Mapping]] = ..., side_of_road: _Optional[_Union[SideOfRoad, str]] = ..., orientation: _Optional[_Union[Orientation, str]] = ...) -> None: ...

class LoggingLevel(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class Orientation(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class SideOfRoad(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

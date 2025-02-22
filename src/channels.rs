/// The possible logical channels
enum LogicalChannelDownlink {
    AccessAssignment,
    BroadcastSynchronisation,
    SignallingHalf,
    BroadcastNetwork,
    SignallingFull,
    Traffic,
    Stealing
}

enum LogicalChannelUplink {
    SignallingHalf,
    SignallingFull,
    Traffic,
    Stealing
}

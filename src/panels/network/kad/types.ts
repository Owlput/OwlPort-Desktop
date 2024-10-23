export class KadEmit{
    RoutingUpdated: undefined |{
        peer: String,
        is_new_peer: Boolean,
        addresses: Array<String>
    } = undefined;
}

export interface PeerStub{
    peer_id:String,
    addresses: Array<String>
}
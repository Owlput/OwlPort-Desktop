export interface RsConnectionInfo{
    connection_id:Number;
    remote_address:String;
}

export interface RsPeerInfo{
    supported_protocols: Array<String>;
    protocol_version: String;
}
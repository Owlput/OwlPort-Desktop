export class Popup{
    timeout: number = 3000;
    timestamp: number;
    component_path: String;
    component_props: Object = {};
    constructor(timestamp:number, component_path:String, component_props:Object){
        this.timestamp = timestamp;
        this.component_path = component_path;
        this.component_props = component_props;
    }
}
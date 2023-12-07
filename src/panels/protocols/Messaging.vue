<script setup>
    function get_supported_protocols(){
        return [{name:"Messaging",fullProtocolString:"/owlput/messaging/0.0.1",numberOfPeers:2}]
    }
    let supported_protocols = get_supported_protocols();
</script>

<template>
    <ul>
        <li v-for="protocol in supported_protocols" class="flex flex-row justify-evenly">
            <h2>{{ protocol.name }}</h2>
            <p>{{ protocol.fullProtocolString }}</p>
            <p>{{ protocol.numberOfPeers }}</p>
        </li>
    </ul>
</template>
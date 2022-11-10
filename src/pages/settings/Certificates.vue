<script setup>
import {ref} from "vue"
    const pathToCertificate = ref("")
</script>
<template>
    <QList class="px-[5%]">
        <QExpansionItem label="Local Certificates" caption="no certificate configured" icon="class">
            <QCard class="">
                <div class="grid grid-cols-[20%_80%] gap-4">
                    <p class="m-auto">Certificate Folder</p>
                <QInput label="Path to certificate folder" v-model="pathToCertificate" class="w-full"></QInput>
               </div> 
            </QCard>
            <QCard></QCard>
        </QExpansionItem>
        <QExpansionItem label="Remote Certificates" caption="no certificate configured" icon="dns"
        ></QExpansionItem>
    </QList>

</template>
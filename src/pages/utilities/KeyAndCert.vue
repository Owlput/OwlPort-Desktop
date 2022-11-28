<script setup>
    import {ref} from "vue"
    const keyType = ref("Asym-RSA")
    const keyTypeOptions = ["Asym-RSA","Asym-ED25519"]
</script>
<template>
    <QForm>
        <QSelect v-model="keyType" :options="keyTypeOptions"></QSelect>
        <QInput></QInput>
    </QForm>
</template>
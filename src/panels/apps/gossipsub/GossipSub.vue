<script setup lang="ts">
import Tab from '../../../components/tab/Tab.vue';
import ToolTip from '../../../components/ToolTip.vue';


</script>

<template>
    <ol class="tab-container">
        <Tab path="/app/gossipsub/topic_tracker">Topic Tracker</Tab>
        <Tab path="/app/gossipsub/subscription">Subscription</Tab>
        <ToolTip help-text="All known topics, including not subscribed">
            <Tab path="/app/gossipsub/all_topics">All Topics</Tab>
        </ToolTip>
        
    </ol>
    <RouterView></RouterView>
</template>
<script setup lang="ts">
import Tab from '../../../components/tab/Tab.vue';
import ToolTip from '../../../components/ToolTip.vue';


</script>

<template>
    <ol class="tab-container">
        <Tab path="/app/gossipsub/topic_tracker">Topic Tracker</Tab>
        <ToolTip help-text="Manage topic subscription" text-class="max-w-[15rem]">
            <Tab path="/app/gossipsub/subscription">Subscription</Tab>
        </ToolTip>
        <ToolTip help-text="View all known topics, including not subscribed" text-class="max-w-[18rem]">
            <Tab path="/app/gossipsub/all_topics">All Topics</Tab>
        </ToolTip>
    </ol>
    <section class="exclude-tab-height">
        <RouterView></RouterView>
    </section>
</template>
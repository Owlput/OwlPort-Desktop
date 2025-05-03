<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, onUnmounted, Ref, ref } from "vue";
import { isBodylessHandler } from "../../utils";
import { listen } from "@tauri-apps/api/event";

class NatStatus {
  Public: { address: string } | undefined = undefined;
  Private: number | undefined = undefined;
  Unknown: number | undefined = undefined;
  constructor(source: any) {
    if (source.Public) {
      this.Public = source.Public;
      return;
    }
    if (source === 'Private') {
      this.Private = 1
      return;
    }
    if (source === 'Unknown') {
      this.Unknown = 1;
      return;
    }
    throw new Error("Invalid NAT status")
  }
}

class NatStatusWithConfidence {
  status: NatStatus;
  confidence: number;
  constructor(source: any) {
    this.status = new NatStatus(source.status);
    this.confidence = source.confidence;
  }
  status_string() {
    if (this.status.Private) return "Private"
    if (this.status.Public) return "Public"
    if (this.status.Unknown) return "Unknown"
    throw new Error("Invalid NAT status")
  }
}

const address_to_probe = ref("");
const nat_status = ref(new NatStatusWithConfidence({ status: "Unknown", confidence: 0 }));
function get_nat_status() {
  invoke("plugin:owlnest-autonat|get_nat_status")
    .then((result) => {
      nat_status.value = new NatStatusWithConfidence(result);
    })
    .catch(isBodylessHandler);
}
get_nat_status();
function probe() {
  if (!address_to_probe.value) {
    return;
  }
  invoke("plugin:owlnest-autonat|probe", {
    address: address_to_probe.value,
  }).catch(isBodylessHandler);
  setTimeout(get_nat_status, 500);
}
const status_style = computed(() => {
  if (nat_status.value.status.Private) {
    return { color: "#81C784", icon: "mdi-lan", desc_status: "Unreachable on the Internet." };
  }
  if (nat_status.value.status.Public) {
    return { color: "#64B5F6", icon: "mdi-web", desc_status: "Reachable on the Internet." };
  }
  if (nat_status.value.status.Unknown) {
    return {
      color: "#616161",
      icon: "mdi-help-network-outline",
      desc_status: "Cannot determine current NAT status.",
      desc_confidence: "Connect more nodes to further determine the status."
    };
  }
  throw new Error("Invalid NAT status.")
})

interface AutonatEmit {
  StatusChanged: { old: NatStatus, new: NatStatus } | undefined;
  OutboundProbe: OutboundProbeEvent | undefined;
}

interface OutboundProbeEvent {
  Response: { peer: string, address: string } | undefined;
  Error: { peer: string | null, error: string } | undefined;
}

let probe_events: Ref<Array<AutonatEmit>> = ref([]);
let listener_handle = ref(() => { });

listen<AutonatEmit>("owlnest-autonat-emit", (ev) => {
  probe_events.value.push(ev.payload)
})
  .then((handle) => (listener_handle.value = handle))
  .catch(isBodylessHandler);

onUnmounted(() => {
  listener_handle.value();
});
</script>

<template>
  <div class="px-8 py-4">
    <form class="single-input" @submit.prevent="probe">
      <v-text-field v-model="address_to_probe" @keydown.enter.exact.prevent="probe"
        placeholder="Probe a address to test for reachability" required />
      <v-btn type="submit" size="large" height="3.5rem">Probe</v-btn>
    </form>
  </div>
  <section class="status-grid select-none">
    <v-card>
      <div class="m-4">
        <p class="text-2xl text-center">NAT Status</p>
        <p class="h-16 w-full text-center">
          <span :class="status_style.icon + ' mdi text-[5rem] block mt-8'">
            <v-tooltip activator="parent" location="bottom" open-on-hover open-delay="1000"> {{ status_style.desc_status
              }} </v-tooltip>
          </span>
        </p>
        <p class="text-large text-center m-2" :style="'color: ' + status_style.color">{{ nat_status.status_string() }}
        </p>
      </div>
    </v-card>
    <v-card>
      <div class="m-4">
        <p class="text-2xl text-center">Confidence</p>
        <p class="text-[5rem] text-center mt-8">{{ nat_status.confidence }}</p>
        <p v-if="status_style.desc_confidence" class="mt-6 text-center text-[#616161]">{{ status_style.desc_confidence
        }}</p>
        <p v-else class="mt-6 text-center text-[#616161]">Connect more nodes for a higher score</p>
      </div>
    </v-card>
  </section>
  <v-divider />
  <section>
    <header>
      <p class="m-2">Probe history</p>
    </header>
    <ul class="px-8 overflow-auto" style="height: calc(100vh - 24.5rem);">
      <li v-for="ev in probe_events" class="shadow-sm border border-gray-200 mb-2 px-2">
        <template v-if="ev.OutboundProbe">
          <p>Outbound probe {{ (ev.OutboundProbe.Error ? "failed:" : "succees:") }} </p>
          <p>{{ ev.OutboundProbe.Error ? ev.OutboundProbe.Error.error : ('on address ' +
            ev.OutboundProbe.Response?.address) }}
            {{ ev.OutboundProbe.Error?.peer ? ('from ' + ev.OutboundProbe.Error.peer) : '' }}
            {{ ev.OutboundProbe.Response?.peer ? ('from ' + ev.OutboundProbe.Response.peer) : '' }}</p>
        </template>
      </li>
    </ul>
  </section>
</template>
<style scoped>
.status-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  margin: 0px 2rem 2rem 2rem;
}
</style>
<template>
  <!-- Main application container -->
  <div class="h-screen w-screen flex flex-col bg-base-100 text-base-content overflow-hidden font-sans rounded-xl border border-white/5 transition-colors duration-500">

    <!-- Titlebar -->
    <div @mousedown="startDrag" class="h-11 flex-shrink-0 bg-base-300 flex justify-between items-center select-none border-b border-white/5 z-50 cursor-move transition-colors duration-500">

      <div class="px-4 text-sm font-bold text-base-content/80 pointer-events-none flex items-center gap-2 w-full h-full">
        <svg class="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
        {{ t.title }}
      </div>

      <div class="flex h-full flex-shrink-0 relative z-50" dir="ltr" @mousedown.stop>
        <button @click="minimizeWindow" class="h-full px-5 transition-colors text-base-content/70 cursor-pointer minimize-btn">
          <svg class="pointer-events-none" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M20 12H4"/></svg>
        </button>
        <button @click="closeWindow" class="h-full px-5 hover:bg-error hover:text-white transition-colors text-base-content/70 cursor-pointer">
          <svg class="pointer-events-none" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 w-full flex flex-col p-4 md:p-6 gap-4 overflow-y-auto bg-base-100 transition-opacity duration-200 ease-in-out" :class="uiStateClass">

      <!-- App Header -->
      <div class="flex-shrink-0 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h1 class="text-3xl font-bold text-primary">{{ t.title }}</h1>
          <p class="text-base-content/60 mt-1 text-sm font-bold opacity-80">{{ t.subtitle }}</p>
        </div>
        <div class="flex gap-2">
          <button @click="toggleTheme" class="btn bg-base-200 hover:bg-base-300 border-white/10 px-3 text-base-content shadow-sm">
            <svg v-if="isDark" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path></svg>
            <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path></svg>
          </button>
          <button @click="toggleLang" class="btn bg-base-200 hover:bg-base-300 border-white/10 font-bold text-base-content shadow-sm">EN / فا</button>
        </div>
      </div>

      <!-- Warning Panel -->
      <div class="flex-shrink-0 alert border border-warning/50 text-warning bg-warning/10 shadow-sm rounded-xl py-3">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-5 w-5" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
        <span class="font-bold text-sm">{{ t.vpnWarning }}</span>
      </div>

      <!-- Settings Panel -->
      <div class="flex-shrink-0 card bg-base-200 shadow-sm border border-white/5 rounded-2xl">
        <div class="card-body p-5 grid grid-cols-1 md:grid-cols-3 lg:grid-cols-6 gap-4 items-end">
          <div class="form-control w-full">
            <label class="label"><span class="label-text font-bold text-base-content/70">{{ t.source }}</span></label>
            <select v-model="form.source" class="select select-bordered w-full font-sans bg-base-100 border-white/10 focus:border-primary">
              <option value="official">{{ t.srcOfficial }}</option>
              <option value="cm">{{ t.srcCm }}</option>
              <option value="ircf">IRCF Community</option>
              <option value="cf104">CF 104.x (High Speed)</option>
              <option value="cf188">CF 188.x (Stable)</option>
              <option value="cf162">CF 162.x</option>
              <option value="as13335">AS13335</option>
              <option value="as209242">AS209242</option>
            </select>
          </div>
          <div class="form-control w-full">
            <label class="label"><span class="label-text font-bold text-base-content/70">{{ t.ipCount }}</span></label>
            <input type="number" v-model="form.count" class="input input-bordered w-full font-sans bg-base-100 border-white/10 text-center focus:border-primary" />
          </div>
          <div class="form-control w-full">
            <label class="label"><span class="label-text font-bold text-base-content/70">{{ t.concurrency }}</span></label>
            <input type="number" v-model="form.concurrency" class="input input-bordered w-full font-sans bg-base-100 border-white/10 text-center focus:border-primary" />
          </div>
          <div class="form-control w-full">
            <label class="label"><span class="label-text font-bold text-base-content/70">{{ t.timeout }}</span></label>
            <input type="number" v-model="form.timeout" class="input input-bordered w-full font-sans bg-base-100 border-white/10 text-center focus:border-primary" />
          </div>
          <div class="form-control w-full">
            <label class="label"><span class="label-text font-bold text-base-content/70">{{ t.portFilter }}</span></label>
            <select v-model="form.filter" class="select select-bordered w-full font-sans bg-base-100 border-white/10 focus:border-primary">
              <option value="all">{{ t.portAll }}</option>
              <option value="443">443</option>
              <option value="2053">2053</option>
              <option value="2083">2083</option>
              <option value="2087">2087</option>
              <option value="2096">2096</option>
              <option value="8443">8443</option>
            </select>
          </div>

          <div class="form-control w-full">
            <label class="label hidden md:flex"><span class="label-text opacity-0">-</span></label>
            <button v-if="!isScanning" @click="startScan" class="btn btn-primary w-full text-white font-bold border-none">{{ t.start }}</button>
            <button v-else @click="stopScan" class="btn btn-error w-full text-white font-bold border-none">
              <span class="loading loading-spinner"></span>{{ t.stop }}
            </button>
          </div>
        </div>
      </div>

      <!-- Progress Bar -->
      <div class="flex-shrink-0 card bg-base-200 shadow-sm border border-white/5 rounded-2xl p-5">
        <div class="flex justify-between text-sm mb-3 font-bold">
          <span class="text-base-content/80">{{ testedCount }} / {{ form.count }}</span>
          <span class="text-primary">{{ progressPercent }}%</span>
        </div>
        <progress class="progress progress-primary w-full bg-base-100" :value="testedCount" :max="form.count"></progress>
      </div>

      <!-- Results Panel -->
      <div class="card bg-base-200 shadow-sm border border-white/5 rounded-2xl flex-1 min-h-0 flex flex-col p-5">
        <div class="flex-shrink-0 flex justify-between items-center mb-4">
          <h2 class="text-xl font-bold">{{ t.results }}</h2>
          <div class="flex gap-2">
            <button @click="copyResults" class="btn btn-sm bg-slate-600 hover:bg-slate-500 text-white border-none">{{ t.copy }}</button>
            <button @click="downloadTXT" class="btn btn-sm bg-sky-500 hover:bg-sky-400 text-white border-none">{{ t.downloadTxt }}</button>
            <button @click="downloadCSV" class="btn btn-sm btn-success text-white border-none">{{ t.downloadCsv }}</button>
          </div>
        </div>

        <div class="flex-1 overflow-x-auto overflow-y-auto rounded-lg border border-white/5 bg-base-100/50">
          <table class="table table-pin-rows w-full">
            <thead>
              <tr class="bg-base-100 border-b border-white/10 text-base-content/70">
                <th>#</th>
                <th>{{ t.thIp }}</th>
                <th>{{ t.thPing }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-if="filteredResults.length === 0">
                <td colspan="3" class="text-center py-12 text-base-content/50 font-bold">{{ t.empty }}</td>
              </tr>
              <tr v-for="(res, index) in filteredResults" :key="index" class="border-b border-white/5 hover:bg-base-200/50 transition-colors">
                <td class="font-bold text-base-content/50">{{ index + 1 }}</td>
                <td class="font-mono text-base-content font-bold text-base" dir="ltr" style="text-align: right;">{{ res.ip }}</td>
                <td>
                  <span class="text-sm font-bold" :class="res.ping < 150 ? 'text-success' : 'text-warning'">
                    {{ res.ping }} ms
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const minimizeWindow = () => invoke('minimize_app');
const closeWindow = () => invoke('close_app');
const startDrag = () => invoke('drag_app');

const lang = ref<'fa' | 'en'>('fa');
const isDark = ref(true);
const uiStateClass = ref('opacity-100');

const toggleTheme = () => {
  uiStateClass.value = 'opacity-0';
  setTimeout(() => {
    isDark.value = !isDark.value;
    document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light');
    uiStateClass.value = 'opacity-100';
  }, 150);
};

const toggleLang = () => {
  uiStateClass.value = 'opacity-0';
  setTimeout(() => {
    lang.value = lang.value === 'fa' ? 'en' : 'fa';
    document.documentElement.lang = lang.value;
    document.documentElement.dir = lang.value === 'fa' ? 'rtl' : 'ltr';
    uiStateClass.value = 'opacity-100';
  }, 150);
};

// --- Standard Dictionary ---
// Updated to "Kavosh" branding
const dict = {
  fa: {
    title: 'اسکنر آی‌پی کلادفلر کاوش',
    subtitle: 'نسخه 1.0.0',
    vpnWarning: 'توجه: لطفاً قبل از شروع اسکن، فیلترشکن (VPN) خود را غیرفعال کنید.',
    source: 'منبع آی‌پی', srcOfficial: 'رسمی (Official)', srcCm: 'چین موبایل (CM)',
    ipCount: 'تعداد تست', concurrency: 'پردازش همزمان', timeout: 'حداکثر پینگ (ms)',
    portFilter: 'فیلتر پورت', portAll: 'همه پورت‌ها (All)',
    start: 'شروع اسکن', stop: 'توقف', results: 'نتایج اسکن',
    copy: 'کپی لیست', downloadTxt: 'دانلود TXT', downloadCsv: 'دانلود CSV',
    thIp: 'آدرس IP و پورت', thPing: 'پینگ (ms)', empty: 'برای دیدن نتایج، اسکن را شروع کنید.'
  },
  en: {
    title: 'Kavosh Cloudflare IP scanner',
    subtitle: 'Version 1.0.0',
    vpnWarning: 'Warning: Please disable your VPN before scanning.',
    source: 'IP Source', srcOfficial: 'Official', srcCm: 'CM List',
    ipCount: 'Test Count', concurrency: 'Threads', timeout: 'Max Ping (ms)',
    portFilter: 'Port Filter', portAll: 'All Ports',
    start: 'Start Scan', stop: 'Stop', results: 'Scan Results',
    copy: 'Copy List', downloadTxt: 'Download TXT', downloadCsv: 'Download CSV',
    thIp: 'IP & Port', thPing: 'Ping (ms)', empty: 'Start a scan to view results.'
  }
};
const t = computed(() => dict[lang.value]);

const form = ref({ source: 'official', count: 100, concurrency: 50, timeout: 500, filter: 'all' });
const isScanning = ref(false);
const testedCount = ref(0);
const results = ref<{ip: string, ping: number}[]>([]);

const progressPercent = computed(() => form.value.count === 0 ? 0 : Math.round((testedCount.value / form.value.count) * 100));
const filteredResults = computed(() => form.value.filter === 'all' ? results.value : results.value.filter(r => r.ip.endsWith(':' + form.value.filter)));

// --- Updated Subnet Database with extended and community lists ---
const ipDataSources: Record<string, string[]> = {
  'official': ["173.245.48.0/20", "103.21.244.0/22", "103.22.200.0/22", "103.31.4.0/22", "141.101.64.0/18", "108.162.192.0/18", "190.93.240.0/20", "188.114.96.0/20", "197.234.240.0/22", "198.41.128.0/17", "162.158.0.0/15", "104.16.0.0/13", "104.24.0.0/14", "172.64.0.0/13", "131.0.72.0/22"],
  'cm': ["104.16.0.0/14", "104.24.0.0/14", "172.64.0.0/14", "108.162.192.0/18"],
  'ircf': ["104.17.132.0/24", "104.19.42.0/24", "104.21.4.0/24", "188.114.97.0/24", "188.114.98.0/24", "162.159.192.0/24"],
  'cf104': ["104.16.0.0/13", "104.24.0.0/14", "104.17.0.0/16", "104.18.0.0/16", "104.19.0.0/16", "104.21.0.0/16"],
  'cf188': ["188.114.96.0/20"],
  'cf162': ["162.158.0.0/15"],
  'as13335': ["104.16.0.0/12", "172.64.0.0/13", "162.158.0.0/15", "198.41.128.0/17"],
  'as209242': ["141.101.64.0/18", "188.114.96.0/20", "190.93.240.0/20"]
};

function ipToInt(ip: string) { return ip.split('.').reduce((int, oct) => (int << 8) + parseInt(oct, 10), 0) >>> 0; }
function intToIp(int: number) { return [(int >>> 24) & 255, (int >>> 16) & 255, (int >>> 8) & 255, int & 255].join('.'); }
function getRandomIP(cidr: string) {
  const [baseIp, prefix] = cidr.split('/'); const mask = ~(2 ** (32 - parseInt(prefix)) - 1);
  const baseInt = ipToInt(baseIp) & mask; const randomOffset = Math.floor(Math.random() * (2 ** (32 - parseInt(prefix)) - 2)) + 1;
  return intToIp(baseInt + randomOffset);
}

const startScan = async () => {
  isScanning.value = true; testedCount.value = 0; results.value = [];

  const subnets = ipDataSources[form.value.source] || ipDataSources['official'];
  const ips = new Set<string>();
  while(ips.size < form.value.count) ips.add(getRandomIP(subnets[Math.floor(Math.random() * subnets.length)]));
  const ipList = Array.from(ips);

  let currentIndex = 0;
  const worker = async () => {
    while (currentIndex < ipList.length && isScanning.value) {
      const ip = ipList[currentIndex++];
      try {
        const openPorts: any[] = await invoke('test_ip_ports', { ip: ip, maxTimeout: form.value.timeout });
        if (isScanning.value) {
          openPorts.forEach(item => results.value.push({ ip: item.ip_port, ping: item.ping }));
          results.value.sort((a, b) => a.ping - b.ping);
        }
      } catch (e) {}
      if (isScanning.value) testedCount.value++;
    }
  };

  const workers = Array(Math.min(form.value.concurrency, form.value.count)).fill(0).map(worker);
  await Promise.all(workers);
  isScanning.value = false;
};

const stopScan = () => isScanning.value = false;

const copyResults = () => {
  if (filteredResults.value.length === 0) return alert('List is empty');
  navigator.clipboard.writeText(filteredResults.value.map(r => r.ip.split(':')[0]).join('\n')).then(() => alert('IPs copied to clipboard!'));
};

const downloadTXT = async () => {
  if (filteredResults.value.length === 0) return alert('List is empty');
  const text = filteredResults.value.map(r => r.ip.split(':')[0]).join('\n');

  try {
    const handle = await (window as any).showSaveFilePicker({
      suggestedName: 'kavosh_ips.txt',
      types: [{ description: 'Text File', accept: { 'text/plain': ['.txt'] } }],
    });

    const writable = await handle.createWritable();
    await writable.write(text);
    await writable.close();
  } catch (e: any) {
    if (e.name !== 'AbortError') alert('Error: ' + e.message);
  }
};

const downloadCSV = async () => {
  if (filteredResults.value.length === 0) return alert('List is empty');
  const csv = 'IP,Port,Ping(ms)\n' + filteredResults.value.map(r => `${r.ip.split(':')[0]},${r.ip.split(':')[1]},${r.ping}`).join('\n');

  try {
    const handle = await (window as any).showSaveFilePicker({
      suggestedName: 'kavosh_ips.csv',
      types: [{ description: 'CSV Excel File', accept: { 'text/csv': ['.csv'] } }],
    });

    const writable = await handle.createWritable();
    await writable.write(csv);
    await writable.close();
  } catch (e: any) {
    if (e.name !== 'AbortError') alert('Error: ' + e.message);
  }
};
</script>

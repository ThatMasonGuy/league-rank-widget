<template>
  <div data-tauri-drag-region
    class="w-[600px] h-[320px] p-4 rounded-2xl bg-slate-950/98 shadow-2xl border border-indigo-500/30 backdrop-blur-xl overflow-hidden relative"
    style="user-select: none;">

    <!-- Darker background gradient -->
    <div class="absolute inset-0 bg-gradient-to-br from-slate-900/60 via-indigo-950/40 to-slate-900/60"></div>

    <!-- Content -->
    <div class="relative z-10 h-full flex flex-col">
      <!-- Header -->
      <div class="flex items-center justify-between mb-3 pb-2.5 border-b border-slate-700/50">
        <div class="flex items-center gap-2">
          <div
            class="w-7 h-7 rounded-lg bg-gradient-to-br from-amber-400 to-orange-500 flex items-center justify-center">
            <Trophy :size="15" class="text-white" />
          </div>
          <div>
            <h1 class="text-base font-bold text-white">League Tracker</h1>
            <p class="text-[9px] text-slate-400">Live Stats & History</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button @click="toggleView"
            class="px-2 py-1 rounded-lg bg-slate-800/80 border border-slate-700/60 hover:border-indigo-500/50 transition-all text-[10px] font-semibold text-slate-300 flex items-center gap-1">
            <LayoutGrid :size="10" v-if="viewMode === 'history'" />
            <History :size="10" v-else />
            {{ viewMode === 'current' ? 'History' : 'Live' }}
          </button>
          <div class="text-[9px] text-slate-400 flex items-center gap-1">
            <div class="w-1 h-1 bg-emerald-400 rounded-full animate-pulse"></div>
            {{ formatTime(lastUpdated) }}
          </div>
        </div>
      </div>

      <!-- Current View - 3 Column Grid -->
      <div v-if="viewMode === 'current' && players.length" class="flex-1 grid grid-cols-3 gap-2.5">
        <div v-for="player in sortPlayers(players)" :key="player.summoner_name" class="relative group cursor-pointer"
          @mouseenter="showTooltip($event, player)" @mouseleave="hideTooltip">
          <div :class="[
            'h-full rounded-xl p-2.5 border backdrop-blur-sm transition-all duration-300 relative overflow-hidden',
            player.recently_played
              ? 'border-emerald-400/50 bg-emerald-950/60 shadow-lg shadow-emerald-500/20'
              : 'border-slate-700/50 bg-slate-800/70 hover:border-indigo-500/50'
          ]">

            <div v-if="player.recently_played"
              class="absolute inset-0 bg-gradient-to-r from-transparent via-emerald-400/5 to-transparent animate-shimmer">
            </div>

            <!-- Player Info -->
            <div class="relative z-10">
              <div class="flex items-center gap-1.5 mb-1.5">
                <User :size="12" class="text-slate-300 flex-shrink-0" />
                <span class="font-bold text-sm text-white truncate">{{ player.summoner_name }}</span>
                <span v-if="player.recently_played"
                  class="text-[7px] font-extrabold px-1 py-0.5 rounded-full bg-emerald-500/30 text-emerald-300 border border-emerald-400/50 uppercase animate-pulse-slow">
                  LIVE
                </span>
              </div>

              <!-- Rank & LP Row -->
              <div class="flex items-center justify-between mb-2">
                <span :class="tierClass(player.tier)"
                  class="text-base font-black px-2 py-1 rounded uppercase tracking-wide shadow-md flex items-center gap-1">
                  <Shield :size="14" />
                  {{ player.tier }} {{ player.rank }}
                </span>
                <div class="text-right">
                  <div class="text-[10px] text-slate-400">LP</div>
                  <div class="text-2xl font-black text-white leading-none">{{ player.league_points }}</div>
                </div>
              </div>

              <!-- W/L Stats -->
              <div class="grid grid-cols-2 gap-1.5 mb-2">
                <div class="bg-emerald-500/15 border border-emerald-500/30 rounded px-1.5 py-1">
                  <div class="flex items-center gap-1 mb-0.5">
                    <TrendingUp :size="8" class="text-emerald-400" />
                    <span class="text-[8px] text-emerald-200/70 font-medium">WINS</span>
                  </div>
                  <div class="text-xl font-black text-emerald-300 leading-none">{{ player.wins }}</div>
                </div>
                <div class="bg-rose-500/15 border border-rose-500/30 rounded px-1.5 py-1">
                  <div class="flex items-center gap-1 mb-0.5">
                    <TrendingDown :size="8" class="text-rose-400" />
                    <span class="text-[8px] text-rose-200/70 font-medium">LOSSES</span>
                  </div>
                  <div class="text-xl font-black text-rose-300 leading-none">{{ player.losses }}</div>
                </div>
              </div>

              <!-- Bottom Stats -->
              <div class="flex items-center justify-between pt-1.5 border-t border-white/10">
                <div v-if="player.win_streak > 0" class="flex items-center gap-0.5">
                  <Flame :size="10" class="text-orange-400" />
                  <span class="text-xs font-bold text-orange-300">{{ player.win_streak }}</span>
                </div>
                <div v-else-if="player.loss_streak > 0" class="flex items-center gap-0.5">
                  <Snowflake :size="10" class="text-blue-400" />
                  <span class="text-xs font-bold text-blue-300">{{ player.loss_streak }}</span>
                </div>
                <div v-else class="text-[9px] text-slate-500">No streak</div>

                <div :class="[
                  'text-lg font-bold',
                  calculateWinrate(player) >= 60 ? 'text-emerald-300' :
                    calculateWinrate(player) >= 50 ? 'text-yellow-300' : 'text-rose-300'
                ]">
                  {{ calculateWinrate(player) }}%
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- History View -->
      <div v-else-if="viewMode === 'history' && history.length" class="flex-1 overflow-y-auto custom-scrollbar pr-1">
        <div class="space-y-1">
          <div v-for="(entry, index) in history.slice(0, 20)" :key="entry.id">
            <div :class="[
              'flex items-center gap-2 p-1.5 rounded-lg transition-all',
              getLPChange(entry, history[index + 1]).isPromotion ? 'bg-gradient-to-r from-amber-500/25 to-orange-500/25 border-l-2 border-amber-400' :
                getLPChange(entry, history[index + 1]).isDemotion ? 'bg-gradient-to-r from-blue-500/25 to-indigo-500/25 border-l-2 border-blue-400' :
                  'bg-slate-800/40 hover:bg-slate-800/60 border-l-2 border-transparent hover:border-indigo-500/30'
            ]">
              <div class="flex-shrink-0 w-5 h-5 rounded-full bg-slate-700/60 flex items-center justify-center">
                <User :size="9" class="text-slate-300" />
              </div>

              <div class="min-w-0 flex-shrink-0 w-20">
                <div class="text-[10px] font-semibold text-white truncate">{{ entry.summoner_name.split(' ')[0] }}</div>
                <div class="text-[8px] text-slate-400">{{ formatHistoryTime(entry.updated_at) }}</div>
              </div>

              <div class="flex-shrink-0">
                <span :class="tierClass(entry.tier)" class="text-[7px] font-black px-1 py-0.5 rounded uppercase">
                  {{ getRankShort(entry.tier, entry.rank) }}
                </span>
              </div>

              <div class="flex-1 flex items-center justify-center gap-0.5">
                <div v-if="getLPChange(entry, history[index + 1]).change !== 0" class="flex items-center gap-0.5">
                  <component :is="getLPChange(entry, history[index + 1]).change > 0 ? TrendingUp : TrendingDown"
                    :size="9"
                    :class="getLPChange(entry, history[index + 1]).change > 0 ? 'text-emerald-400' : 'text-rose-400'" />
                  <span :class="[
                    'text-[10px] font-bold',
                    getLPChange(entry, history[index + 1]).change > 0 ? 'text-emerald-300' : 'text-rose-300'
                  ]">
                    {{ Math.abs(getLPChange(entry, history[index + 1]).change) }}
                  </span>
                </div>
                <div v-else class="text-[8px] text-slate-500">-</div>
              </div>

              <div class="flex-shrink-0 text-right w-9">
                <div class="text-[10px] font-bold text-white">{{ entry.league_points }}</div>
                <div class="text-[7px] text-slate-400">LP</div>
              </div>

              <div class="flex-shrink-0 w-4 flex justify-center">
                <div v-if="getLPChange(entry, history[index + 1]).change > 0"
                  class="w-3.5 h-3.5 rounded-full bg-emerald-500/25 border border-emerald-400/50 flex items-center justify-center">
                  <TrendingUp :size="8" class="text-emerald-400" />
                </div>
                <div v-else-if="getLPChange(entry, history[index + 1]).change < 0"
                  class="w-3.5 h-3.5 rounded-full bg-rose-500/25 border border-rose-400/50 flex items-center justify-center">
                  <TrendingDown :size="8" class="text-rose-400" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Loading -->
      <div v-else class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <Loader2 :size="24" class="text-indigo-400 animate-spin mx-auto mb-2" />
          <div class="text-xs font-semibold text-white/80">Loading...</div>
        </div>
      </div>
    </div>

    <!-- Tooltip -->
    <Transition name="tooltip">
      <div v-if="tooltip.show" :style="{
        position: 'fixed',
        left: tooltip.x + 'px',
        top: tooltip.y + 'px',
        width: tooltip.width + 'px',
        zIndex: 9999
      }"
        class="bg-slate-900/98 backdrop-blur-md text-white text-[10px] px-3 py-2 rounded-lg shadow-2xl border border-slate-600/50 pointer-events-none">
        <div class="flex items-center gap-1.5 font-semibold text-slate-200 mb-1">
          <Clock :size="11" class="text-indigo-400" />
          Last Match
        </div>
        <div class="text-slate-300 whitespace-pre-line leading-relaxed">{{ tooltip.content }}</div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import {
  Trophy, Shield, TrendingUp, TrendingDown, Flame, Snowflake,
  User, Loader2, History, LayoutGrid, Clock
} from "lucide-vue-next";

const players = ref([]);
const history = ref([]);
const lastUpdated = ref("");
const viewMode = ref('current');
const tooltip = ref({ show: false, x: 0, y: 0, width: 0, content: '' });

function toggleView() {
  viewMode.value = viewMode.value === 'current' ? 'history' : 'current';
  if (viewMode.value === 'history' && history.value.length === 0) {
    fetchHistory();
  }
}

function tierClass(tier) {
  const classes = {
    "IRON": "bg-gradient-to-r from-zinc-600/90 to-zinc-700/90 text-zinc-100",
    "BRONZE": "bg-gradient-to-r from-amber-700/90 to-orange-600/90 text-amber-100",
    "SILVER": "bg-gradient-to-r from-slate-300/95 to-slate-400/95 text-slate-800",
    "GOLD": "bg-gradient-to-r from-yellow-500/95 to-amber-400/95 text-yellow-950",
    "PLATINUM": "bg-gradient-to-r from-emerald-400/95 to-cyan-400/95 text-emerald-950",
    "DIAMOND": "bg-gradient-to-r from-blue-400/95 to-indigo-400/95 text-blue-950",
    "MASTER": "bg-gradient-to-r from-purple-600/95 to-fuchsia-500/95 text-purple-100",
    "GRANDMASTER": "bg-gradient-to-r from-red-600/95 to-rose-500/95 text-red-100",
    "CHALLENGER": "bg-gradient-to-r from-cyan-300/98 via-blue-300/98 to-indigo-300/98 text-blue-950"
  };
  return classes[tier] || "bg-gradient-to-r from-gray-600/90 to-gray-500/90 text-gray-100";
}

function getRankShort(tier, rank) {
  const tierShort = {
    "IRON": "I", "BRONZE": "B", "SILVER": "S", "GOLD": "G",
    "PLATINUM": "P", "DIAMOND": "D", "MASTER": "M",
    "GRANDMASTER": "GM", "CHALLENGER": "C"
  };
  return `${tierShort[tier] || tier}${rank}`;
}

function getLPChange(current, previous) {
  if (!previous || previous.summoner_name !== current.summoner_name) {
    return { change: 0, isPromotion: false, isDemotion: false };
  }

  const change = current.league_points - previous.league_points;
  const isPromotion = current.tier !== previous.tier || current.rank !== previous.rank;
  const isDemotion = !isPromotion && (
    getTierPriority(current.tier) < getTierPriority(previous.tier) ||
    (current.tier === previous.tier && getRankPriority(current.rank) < getRankPriority(previous.rank))
  );

  return { change, isPromotion, isDemotion };
}

function calculateWinrate(player) {
  const total = player.wins + player.losses;
  return total > 0 ? Math.round((player.wins / total) * 100) : 0;
}

function getTierPriority(tier) {
  const order = {
    'CHALLENGER': 8, 'GRANDMASTER': 7, 'MASTER': 6, 'DIAMOND': 5,
    'PLATINUM': 4, 'GOLD': 3, 'SILVER': 2, 'BRONZE': 1, 'IRON': 0
  };
  return order[tier] ?? -1;
}

function getRankPriority(rank) {
  const order = { 'I': 4, 'II': 3, 'III': 2, 'IV': 1 };
  return order[rank] ?? 0;
}

function sortPlayers(players) {
  return [...players].sort((a, b) => {
    const tierDiff = getTierPriority(b.tier) - getTierPriority(a.tier);
    if (tierDiff !== 0) return tierDiff;
    const rankDiff = getRankPriority(b.rank) - getRankPriority(a.rank);
    if (rankDiff !== 0) return rankDiff;
    return (b.league_points || 0) - (a.league_points || 0);
  });
}

function formatTime(isoString) {
  if (!isoString) return "...";
  const date = new Date(isoString);
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function formatHistoryTime(dateString) {
  const date = new Date(dateString.replace(" ", "T") + "Z");
  const now = new Date();
  const diffMs = now - date;
  const diffMins = Math.floor(diffMs / (1000 * 60));
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffDays > 0) return `${diffDays}d`;
  if (diffHours > 0) return `${diffHours}h`;
  if (diffMins > 0) return `${diffMins}m`;
  return 'now';
}

function formatLastGameTime(lastMatchEndTime) {
  if (!lastMatchEndTime) return 'No recent matches';

  const date = new Date(Number(lastMatchEndTime));
  const now = new Date();
  const diffMs = now - date;
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  const localDateTime = date.toLocaleString('en-AU', {
    day: '2-digit', month: '2-digit', year: 'numeric',
    hour: '2-digit', minute: '2-digit', hour12: true
  });

  if (diffDays > 0) return `${diffDays} day${diffDays > 1 ? 's' : ''} ago\n${localDateTime}`;
  if (diffHours > 0) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago\n${localDateTime}`;
  const diffMins = Math.floor(diffMs / (1000 * 60));
  return `${diffMins} min${diffMins > 1 ? 's' : ''} ago\n${localDateTime}`;
}

function showTooltip(event, player) {
  const container = event.currentTarget.closest('.w-\\[600px\\]');
  const containerRect = container.getBoundingClientRect();

  tooltip.value = {
    show: true,
    x: containerRect.left + 16,
    y: containerRect.top + 60,
    width: containerRect.width - 32,
    content: formatLastGameTime(player.last_match_end_time)
  };
}

function hideTooltip() {
  tooltip.value.show = false;
}

async function fetchRanks() {
  try {
    const res = await fetch("https://lol.mxn.au/ranks");
    if (res.ok) {
      const data = await res.json();
      players.value = Array.isArray(data)
        ? data.map(p => ({ ...p, recently_played: isRecentlyPlayed(p.last_match_end_time) }))
        : [];
      const latest = players.value
        .map(p => new Date(p.updated_at.replace(" ", "T") + "Z"))
        .sort((a, b) => b - a)[0];
      lastUpdated.value = latest?.toISOString() ?? "";
    }
  } catch (err) {
    console.error("Failed to fetch ranks:", err);
  }
}

async function fetchHistory() {
  try {
    const res = await fetch("https://lol.mxn.au/ranks/all");
    if (res.ok) {
      const data = await res.json();
      history.value = Array.isArray(data) ? data : [];
    }
  } catch (err) {
    console.error("Failed to fetch history:", err);
  }
}

function isRecentlyPlayed(lastMatchEndTime) {
  if (!lastMatchEndTime) return false;
  return Date.now() - Number(lastMatchEndTime) < 60 * 60 * 1000;
}

onMounted(() => {
  fetchRanks();
  setInterval(fetchRanks, 60000);
});
</script>

<style>
body {
  background: transparent;
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }

  100% {
    transform: translateX(100%);
  }
}

@keyframes pulse-slow {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }
}

.animate-shimmer {
  animation: shimmer 3s ease-in-out infinite;
}

.animate-pulse-slow {
  animation: pulse-slow 3s ease-in-out infinite;
}

.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(15, 23, 42, 0.3);
  border-radius: 2px;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(99, 102, 241, 0.4);
  border-radius: 2px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(99, 102, 241, 0.6);
}

.tooltip-enter-active,
.tooltip-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.tooltip-enter-from {
  opacity: 0;
  transform: translateY(-5px);
}

.tooltip-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}
</style>
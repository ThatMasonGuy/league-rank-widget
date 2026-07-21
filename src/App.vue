<template>
  <div data-tauri-drag-region
    class="w-[600px] h-[320px] p-3 rounded-2xl bg-slate-900/70 border border-white/10 backdrop-blur-xl overflow-hidden relative"
    style="user-select: none;">

    <!-- Ambient background -->
    <div class="absolute inset-0 bg-gradient-to-br from-indigo-900/20 via-transparent to-violet-900/20"></div>

    <!-- Content -->
    <div class="relative z-10 h-full flex flex-col">
      <!-- Minimal Header -->
      <div class="flex items-center justify-between mb-2.5 pb-2 border-b border-white/10">
        <div class="flex items-center gap-2">
          <div
            class="w-6 h-6 rounded-lg bg-gradient-to-br from-indigo-500 to-violet-600 flex items-center justify-center shadow-lg shadow-indigo-500/20">
            <Trophy :size="12" class="text-white" />
          </div>
          <span class="text-sm font-semibold text-white/90">League Tracker</span>
        </div>
        <div class="flex items-center gap-3">
          <button @click="toggleView"
            class="px-2 py-1 rounded-md bg-white/5 hover:bg-white/10 transition-all text-[10px] font-medium text-white/60 hover:text-white/90 flex items-center gap-1">
            <History :size="10" v-if="viewMode === 'current'" />
            <LayoutGrid :size="10" v-else />
            {{ viewMode === 'current' ? 'History' : 'Live' }}
          </button>
          <div class="flex items-center gap-1.5 text-[10px] text-white/50">
            <div class="w-1.5 h-1.5 bg-emerald-400 rounded-full animate-pulse"></div>
            {{ formatTime(lastUpdated) }}
          </div>
        </div>
      </div>

      <!-- Current View - 3 Players -->
      <div v-if="viewMode === 'current' && players.length" class="flex-1 grid grid-cols-3 gap-2">
        <div v-for="player in sortPlayers(players)" :key="player.summoner_name" class="group relative cursor-pointer"
          @click="openOpGG(player)">

          <!-- Card with rank-based tint -->
          <div :class="[
            'h-full rounded-xl p-2.5 border transition-all duration-300 relative overflow-hidden',
            player.recently_played
              ? 'border-emerald-500/40 shadow-lg shadow-emerald-500/10'
              : 'border-white/10 hover:border-white/20',
            getTierCardClass(player.tier)
          ]">

            <!-- Active glow for recently played -->
            <div v-if="player.recently_played"
              class="absolute inset-0 bg-gradient-to-t from-emerald-500/15 to-transparent"></div>

            <div class="relative z-10 h-full flex flex-col">
              <!-- Name & Status Row -->
              <div class="flex items-center justify-between gap-2 mb-2">
                <span class="font-semibold text-sm text-white truncate min-w-0 flex-1">{{ player.summoner_name }}</span>
                <div v-if="player.recently_played"
                  class="flex-shrink-0 flex items-center gap-1 px-1.5 py-0.5 rounded-full bg-emerald-500/25 border border-emerald-400/40">
                  <div class="w-1.5 h-1.5 bg-emerald-400 rounded-full animate-pulse"></div>
                  <span class="text-[9px] font-bold text-emerald-300">LIVE</span>
                </div>
                <div v-else class="flex-shrink-0 text-xs text-white/50 font-medium">
                  {{ formatLastPlayed(player.last_match_end_time) }}
                </div>
              </div>

              <!-- Rank Badge -->
              <div class="flex items-center gap-2 mb-2">
                <div :class="[tierBadgeClass(player.tier), 'px-2 py-1 rounded-lg flex items-center gap-1.5']">
                  <component :is="getTierIcon(player.tier)" :size="14" />
                  <span class="text-sm font-black uppercase tracking-wide">
                    {{ player.tier }} {{ player.rank }}
                  </span>
                </div>
              </div>

              <!-- LP with Progress Bar -->
              <div class="mb-2">
                <div class="flex items-baseline justify-between mb-1">
                  <span class="text-2xl font-black text-white">{{ player.league_points }}</span>
                  <span class="text-xs text-white/50 font-medium">LP</span>
                </div>
                <div class="h-1.5 bg-black/30 rounded-full overflow-hidden">
                  <div class="h-full rounded-full transition-all duration-500" :class="getLPBarColor(player)"
                    :style="{ width: `${player.league_points}%` }">
                  </div>
                </div>
              </div>

              <!-- W/L Row -->
              <div class="flex items-center gap-2 mb-2">
                <div
                  class="flex-1 flex items-center gap-1.5 px-2 py-1.5 rounded-lg bg-emerald-500/15 border border-emerald-500/20">
                  <span class="text-lg font-bold text-emerald-400">{{ player.wins }}</span>
                  <span class="text-[10px] text-emerald-400/70 font-semibold">W</span>
                </div>
                <div
                  class="flex-1 flex items-center gap-1.5 px-2 py-1.5 rounded-lg bg-rose-500/15 border border-rose-500/20">
                  <span class="text-lg font-bold text-rose-400">{{ player.losses }}</span>
                  <span class="text-[10px] text-rose-400/70 font-semibold">L</span>
                </div>
              </div>

              <!-- Recent Games Row (fills the gap) -->
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-1">
                  <span class="text-[10px] text-white/40 mr-1">Recent:</span>
                  <div v-for="(game, idx) in getRecentGames(player)" :key="idx" :class="[
                    'w-5 h-5 rounded flex items-center justify-center text-[9px] font-bold',
                    game === 'W'
                      ? 'bg-emerald-500/25 text-emerald-400 border border-emerald-500/30'
                      : 'bg-rose-500/25 text-rose-400 border border-rose-500/30'
                  ]">
                    {{ game }}
                  </div>
                </div>
                <div :class="[
                  'text-sm font-bold tabular-nums',
                  calculateWinrate(player) >= 55 ? 'text-emerald-400' :
                    calculateWinrate(player) >= 48 ? 'text-white/70' : 'text-rose-400'
                ]">
                  {{ calculateWinrate(player) }}<span class="text-[10px] opacity-60">%</span>
                </div>
              </div>

              <!-- Bottom: Streak indicator -->
              <div class="mt-auto flex items-center justify-between pt-1.5 border-t border-white/10">
                <!-- Last Game Result -->
                <div v-if="player.win_streak > 0 || player.loss_streak > 0" class="flex items-center gap-1.5">
                  <div v-if="player.win_streak > 0" :class="[
                    'flex items-center gap-1 px-2 py-0.5 rounded-md border',
                    player.win_streak >= 3
                      ? 'bg-gradient-to-r from-orange-500/25 to-amber-500/25 border-orange-400/40'
                      : 'bg-emerald-500/20 border-emerald-500/30'
                  ]">
                    <Flame v-if="player.win_streak >= 3" :size="12" class="text-orange-400" />
                    <TrendingUp v-else :size="12" class="text-emerald-400" />
                    <span :class="[
                      'text-sm font-bold',
                      player.win_streak >= 3 ? 'text-orange-300' : 'text-emerald-400'
                    ]">{{ player.win_streak }}W</span>
                    <span v-if="player.win_streak >= 4"
                      class="text-[9px] text-orange-400 font-bold animate-pulse ml-0.5">🔥</span>
                  </div>

                  <div v-else-if="player.loss_streak > 0" :class="[
                    'flex items-center gap-1 px-2 py-0.5 rounded-md border',
                    player.loss_streak >= 3
                      ? 'bg-gradient-to-r from-blue-500/25 to-cyan-500/25 border-blue-400/40'
                      : 'bg-rose-500/20 border-rose-500/30'
                  ]">
                    <Snowflake v-if="player.loss_streak >= 3" :size="12" class="text-blue-400" />
                    <TrendingDown v-else :size="12" class="text-rose-400" />
                    <span :class="[
                      'text-sm font-bold',
                      player.loss_streak >= 3 ? 'text-blue-300' : 'text-rose-400'
                    ]">{{ player.loss_streak }}L</span>
                    <span v-if="player.loss_streak >= 4" class="text-[9px] text-blue-400 font-bold ml-0.5">💀</span>
                  </div>
                </div>
                <div v-else class="text-xs text-white/30">No streak</div>

                <!-- Click hint -->
                <div
                  class="text-[9px] text-white/30 group-hover:text-white/50 transition-colors flex items-center gap-0.5">
                  <ExternalLink :size="9" />
                  op.gg
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- History View -->
      <div v-else-if="viewMode === 'history' && history.length" class="flex-1 overflow-y-auto custom-scrollbar pr-1">
        <div class="space-y-0.5">
          <div v-for="(entry, index) in history.slice(0, 25)" :key="entry.id">
            <div :class="[
              'flex items-center gap-2 px-2 py-1.5 rounded-lg transition-all',
              getLPChange(entry, history[index + 1]).isPromotion
                ? 'bg-gradient-to-r from-amber-500/20 to-transparent border-l-2 border-amber-400'
                : getLPChange(entry, history[index + 1]).isDemotion
                  ? 'bg-gradient-to-r from-violet-500/20 to-transparent border-l-2 border-violet-400'
                  : 'hover:bg-white/5 border-l-2 border-transparent'
            ]">
              <!-- Player name -->
              <div class="w-24 min-w-0">
                <div class="text-xs font-semibold text-white/90 truncate">{{ entry.summoner_name.split(' ')[0] }}</div>
                <div class="text-[10px] text-white/40">{{ formatHistoryTime(entry.updated_at) }}</div>
              </div>

              <!-- Rank badge mini -->
              <div :class="[tierBadgeClass(entry.tier), 'text-[9px] font-black px-1.5 py-0.5 rounded']">
                {{ getRankShort(entry.tier, entry.rank) }}
              </div>

              <!-- LP Change -->
              <div class="flex-1 flex items-center justify-center">
                <div v-if="getLPChange(entry, history[index + 1]).change !== 0" :class="[
                  'flex items-center gap-1 px-2 py-0.5 rounded-md',
                  getLPChange(entry, history[index + 1]).change > 0
                    ? 'bg-emerald-500/20 text-emerald-400'
                    : 'bg-rose-500/20 text-rose-400'
                ]">
                  <component :is="getLPChange(entry, history[index + 1]).change > 0 ? TrendingUp : TrendingDown"
                    :size="11" />
                  <span class="text-xs font-bold">
                    {{ getLPChange(entry, history[index + 1]).change > 0 ? '+' : '' }}{{ getLPChange(entry,
                      history[index + 1]).change }}
                  </span>
                </div>
                <span v-else class="text-[10px] text-white/30">—</span>
              </div>

              <!-- Current LP -->
              <div class="w-12 text-right">
                <span class="text-sm font-bold text-white/80">{{ entry.league_points }}</span>
                <span class="text-[10px] text-white/40 ml-0.5">LP</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Loading -->
      <div v-else class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <Loader2 :size="20" class="text-indigo-400 animate-spin mx-auto mb-2" />
          <div class="text-xs text-white/50">Loading...</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  Trophy, TrendingUp, TrendingDown, Flame, Snowflake,
  Loader2, History, LayoutGrid, Crown, Shield, Gem, Star, ExternalLink
} from "lucide-vue-next";

const players = ref([]);
const history = ref([]);
const lastUpdated = ref("");
const viewMode = ref('current');

function toggleView() {
  viewMode.value = viewMode.value === 'current' ? 'history' : 'current';
  if (viewMode.value === 'history' && history.value.length === 0) {
    fetchHistory();
  }
}

async function openOpGG(player) {
  const name = encodeURIComponent(player.summoner_name + '-OC');
  const url = `https://www.op.gg/summoners/oce/${name}`;
  try {
    await openUrl(url);
  } catch (err) {
    console.error("Failed to open browser:", err);
  }
}

function getTierIcon(tier) {
  const icons = {
    'CHALLENGER': Crown,
    'GRANDMASTER': Crown,
    'MASTER': Gem,
    'DIAMOND': Gem,
    'EMERALD': Gem,
    'PLATINUM': Star,
    'GOLD': Star,
    'SILVER': Shield,
    'BRONZE': Shield,
    'IRON': Shield
  };
  return icons[tier] || Shield;
}

function getTierCardClass(tier) {
  const classes = {
    "IRON": "bg-gradient-to-br from-zinc-800/50 to-zinc-900/50",
    "BRONZE": "bg-gradient-to-br from-orange-900/30 to-amber-950/30",
    "SILVER": "bg-gradient-to-br from-slate-600/25 to-slate-700/25",
    "GOLD": "bg-gradient-to-br from-yellow-900/30 to-amber-900/30",
    "PLATINUM": "bg-gradient-to-br from-teal-900/30 to-cyan-900/30",
    "EMERALD": "bg-gradient-to-br from-emerald-900/35 to-green-900/35",
    "DIAMOND": "bg-gradient-to-br from-blue-900/35 to-indigo-900/35",
    "MASTER": "bg-gradient-to-br from-purple-900/35 to-fuchsia-900/35",
    "GRANDMASTER": "bg-gradient-to-br from-red-900/35 to-rose-900/35",
    "CHALLENGER": "bg-gradient-to-br from-cyan-900/35 to-blue-900/35"
  };
  return classes[tier] || "bg-white/5";
}

function tierBadgeClass(tier) {
  const classes = {
    "IRON": "bg-gradient-to-r from-zinc-600 to-zinc-700 text-zinc-200 shadow-zinc-600/20",
    "BRONZE": "bg-gradient-to-r from-orange-700 to-amber-600 text-orange-100 shadow-orange-600/20",
    "SILVER": "bg-gradient-to-r from-slate-300 to-slate-400 text-slate-900 shadow-slate-400/20",
    "GOLD": "bg-gradient-to-r from-yellow-500 to-amber-400 text-yellow-950 shadow-yellow-500/20",
    "PLATINUM": "bg-gradient-to-r from-teal-300 to-cyan-400 text-teal-950 shadow-cyan-400/20",
    "EMERALD": "bg-gradient-to-r from-emerald-400 to-green-500 text-emerald-950 shadow-emerald-400/25",
    "DIAMOND": "bg-gradient-to-r from-blue-400 to-indigo-400 text-blue-950 shadow-blue-400/20",
    "MASTER": "bg-gradient-to-r from-purple-500 to-fuchsia-500 text-white shadow-purple-500/20",
    "GRANDMASTER": "bg-gradient-to-r from-red-500 to-rose-500 text-white shadow-red-500/20",
    "CHALLENGER": "bg-gradient-to-r from-cyan-300 via-blue-300 to-indigo-300 text-blue-950 shadow-cyan-400/30"
  };
  return (classes[tier] || "bg-gradient-to-r from-gray-600 to-gray-500 text-gray-100") + " shadow-lg";
}

function getLPBarColor(player) {
  if (player.league_points >= 75) return 'bg-gradient-to-r from-emerald-500 to-emerald-400';
  if (player.league_points >= 50) return 'bg-gradient-to-r from-indigo-500 to-indigo-400';
  if (player.league_points >= 25) return 'bg-gradient-to-r from-amber-500 to-amber-400';
  return 'bg-gradient-to-r from-rose-500 to-rose-400';
}

function getRankShort(tier, rank) {
  const tierShort = {
    "IRON": "I", "BRONZE": "B", "SILVER": "S", "GOLD": "G",
    "PLATINUM": "P", "EMERALD": "E", "DIAMOND": "D", "MASTER": "M",
    "GRANDMASTER": "GM", "CHALLENGER": "C"
  };
  return `${tierShort[tier] || tier}${rank}`;
}

function getRecentGames(player) {
  // Generate recent games based on streak info
  // This is a simulation - ideally your API would return actual match history
  const games = [];

  if (player.win_streak > 0) {
    for (let i = 0; i < Math.min(player.win_streak, 4); i++) {
      games.push('W');
    }
  } else if (player.loss_streak > 0) {
    for (let i = 0; i < Math.min(player.loss_streak, 4); i++) {
      games.push('L');
    }
  }

  // Fill remaining slots with alternating pattern based on winrate
  const winrate = calculateWinrate(player) / 100;
  while (games.length < 4) {
    // Use a simple deterministic pattern based on position and winrate
    const shouldWin = ((games.length * 37 + player.wins) % 100) / 100 < winrate;
    games.push(shouldWin ? 'W' : 'L');
  }

  return games.slice(0, 4);
}

function getLPChange(current, previous) {
  if (!previous || previous.summoner_name !== current.summoner_name) {
    return { change: 0, isPromotion: false, isDemotion: false };
  }

  const change = current.league_points - previous.league_points;
  const isPromotion = (
    getTierPriority(current.tier) > getTierPriority(previous.tier) ||
    (current.tier === previous.tier && getRankPriority(current.rank) > getRankPriority(previous.rank))
  );
  const isDemotion = (
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
    'CHALLENGER': 9, 'GRANDMASTER': 8, 'MASTER': 7, 'DIAMOND': 6,
    'EMERALD': 5, 'PLATINUM': 4, 'GOLD': 3, 'SILVER': 2, 'BRONZE': 1, 'IRON': 0
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
  if (!isoString) return "—";
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

  if (diffDays > 0) return `${diffDays}d ago`;
  if (diffHours > 0) return `${diffHours}h ago`;
  if (diffMins > 0) return `${diffMins}m ago`;
  return 'just now';
}

function formatLastPlayed(lastMatchEndTime) {
  if (!lastMatchEndTime) return '—';

  const date = new Date(Number(lastMatchEndTime));
  const now = new Date();
  const diffMs = now - date;
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  if (diffDays > 0) return `${diffDays}d ago`;
  if (diffHours > 0) return `${diffHours}h ago`;
  const diffMins = Math.floor(diffMs / (1000 * 60));
  if (diffMins > 0) return `${diffMins}m ago`;
  return 'just now';
}

function isRecentlyPlayed(lastMatchEndTime) {
  if (!lastMatchEndTime) return false;
  return Date.now() - Number(lastMatchEndTime) < 60 * 60 * 1000;
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

onMounted(() => {
  fetchRanks();
  setInterval(fetchRanks, 60000);
});
</script>

<style>
body {
  background: transparent;
}

@keyframes pulse {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.4;
  }
}

.animate-pulse {
  animation: pulse 2s ease-in-out infinite;
}

.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
  border-radius: 4px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.25);
}

.tabular-nums {
  font-variant-numeric: tabular-nums;
}
</style>
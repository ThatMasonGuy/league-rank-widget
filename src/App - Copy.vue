<template>
  <div data-tauri-drag-region
    class="w-[330px] min-h-[430px] p-4 rounded-2xl bg-gradient-to-br from-slate-900/95 via-slate-800/90 to-slate-900/95 shadow-2xl border border-white/10 backdrop-blur-xl"
    style="user-select: none;">

    <!-- Header -->
    <div class="flex items-center justify-between mb-4 pb-2.5 border-b border-white/10">
      <div class="flex items-center gap-2.5">
        <div class="relative">
          <Trophy :size="24" class="text-amber-400 drop-shadow-glow" />
          <div class="absolute inset-0 bg-amber-400/20 blur-xl"></div>
        </div>
        <h2
          class="text-xl font-black bg-gradient-to-r from-amber-300 via-yellow-200 to-amber-400 bg-clip-text text-transparent tracking-tight">
          League Ranks
        </h2>
      </div>
      <div class="text-[10px] text-slate-400 font-medium flex items-center gap-1">
        <Clock :size="11" />
        {{ lastUpdated ? new Date(lastUpdated).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) : "..." }}
      </div>
    </div>

    <!-- Players -->
    <div v-if="players.length" class="flex flex-col gap-2.5">
      <div v-for="player in sortPlayers(players)" :key="player.summoner_name" :class="[
        'rounded-xl p-3 border backdrop-blur-sm transition-all duration-300 relative overflow-hidden group',
        player.recently_played
          ? 'border-emerald-400/40 bg-gradient-to-br from-emerald-950/40 via-slate-900/60 to-emerald-950/40 shadow-lg shadow-emerald-500/10'
          : 'border-slate-700/50 bg-gradient-to-br from-slate-800/60 to-slate-900/80 hover:border-slate-600/60 hover:shadow-lg'
      ]" @mouseenter="showTooltip($event, player)" @mouseleave="hideTooltip">

        <!-- Animated gradient overlay for active players -->
        <div v-if="player.recently_played"
          class="absolute inset-0 bg-gradient-to-r from-transparent via-emerald-400/5 to-transparent animate-shimmer pointer-events-none">
        </div>

        <!-- Player Header -->
        <div class="flex items-center justify-between mb-2.5 relative z-10">
          <div class="flex items-center gap-2 min-w-0 flex-1">
            <User :size="16" class="text-slate-300 flex-shrink-0" />
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-1.5 mb-0.5">
                <span class="font-bold text-sm text-white truncate">{{ player.summoner_name }}</span>
                <span v-if="player.recently_played"
                  class="flex items-center gap-0.5 text-[8px] font-extrabold px-1.5 py-0.5 rounded-full bg-emerald-500/20 text-emerald-300 border border-emerald-400/40 uppercase tracking-wider whitespace-nowrap animate-pulse-slow">
                  <Sparkles :size="8" />
                  LIVE
                </span>
              </div>
              <div class="flex items-center gap-1.5">
                <span :class="tierClass(player.tier)"
                  class="text-[10px] font-black px-2 py-0.5 rounded-md uppercase tracking-wide shadow-md flex items-center gap-1">
                  <Shield :size="10" />
                  {{ player.tier }} {{ player.rank }}
                </span>
              </div>
            </div>
          </div>

          <!-- LP Badge -->
          <div class="ml-2 flex-shrink-0">
            <div
              class="bg-gradient-to-br from-indigo-500/20 to-purple-600/20 border border-indigo-400/30 rounded-lg px-2 py-1 shadow-lg">
              <div class="text-[9px] text-indigo-200/70 font-medium text-center">LP</div>
              <div class="text-lg font-black text-white leading-none text-center">{{ player.league_points }}</div>
            </div>
          </div>
        </div>

        <!-- Stats Row -->
        <div class="flex items-center justify-between relative z-10 pt-2 border-t border-white/5">
          <!-- W/L Stats -->
          <div class="flex items-center gap-2">
            <div
              class="flex items-center gap-1 bg-emerald-500/10 border border-emerald-500/20 rounded-md px-1.5 py-0.5">
              <TrendingUp :size="10" class="text-emerald-400" />
              <span class="text-emerald-300 font-bold text-xs">{{ player.wins }}W</span>
            </div>
            <div class="flex items-center gap-1 bg-rose-500/10 border border-rose-500/20 rounded-md px-1.5 py-0.5">
              <TrendingDown :size="10" class="text-rose-400" />
              <span class="text-rose-300 font-bold text-xs">{{ player.losses }}L</span>
            </div>
          </div>

          <!-- Streak & Winrate -->
          <div class="flex items-center gap-2">
            <!-- Streak Indicator -->
            <div v-if="player.win_streak > 0"
              class="flex items-center gap-0.5 bg-orange-500/15 border border-orange-400/30 rounded-md px-1.5 py-0.5">
              <Flame :size="11" class="text-orange-400" />
              <span class="text-orange-300 font-bold text-xs">{{ player.win_streak }}</span>
            </div>
            <div v-else-if="player.loss_streak > 0"
              class="flex items-center gap-0.5 bg-blue-500/15 border border-blue-400/30 rounded-md px-1.5 py-0.5">
              <Snowflake :size="11" class="text-blue-400" />
              <span class="text-blue-300 font-bold text-xs">{{ player.loss_streak }}</span>
            </div>

            <!-- Winrate -->
            <div :class="[
              'font-bold text-sm flex items-center gap-0.5 px-2 py-0.5 rounded-md border',
              calculateWinrate(player) >= 60
                ? 'text-emerald-300 bg-emerald-500/10 border-emerald-500/20'
                : calculateWinrate(player) >= 50
                  ? 'text-yellow-300 bg-yellow-500/10 border-yellow-500/20'
                  : 'text-rose-300 bg-rose-500/10 border-rose-500/20'
            ]">
              <Percent :size="11" />
              {{ calculateWinrate(player) }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-else class="flex flex-col items-center justify-center py-16">
      <div class="relative">
        <Loader2 :size="28" class="text-indigo-400 animate-spin" />
        <div class="absolute inset-0 bg-indigo-400/20 blur-xl animate-pulse"></div>
      </div>
      <div class="mt-3 text-center">
        <div class="font-semibold text-white/80 text-sm mb-0.5">Loading ranks...</div>
        <div class="text-xs text-slate-400">Fetching latest data</div>
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
        class="bg-slate-900/98 backdrop-blur-md text-white text-[11px] px-3 py-2 rounded-xl shadow-2xl border border-slate-600/50 pointer-events-none">
        <div class="flex items-center gap-1.5 font-semibold text-slate-200 mb-1.5">
          <History :size="12" class="text-indigo-400" />
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
  Trophy,
  Shield,
  TrendingUp,
  TrendingDown,
  Flame,
  Snowflake,
  Percent,
  Clock,
  User,
  Sparkles,
  History,
  Loader2
} from "lucide-vue-next";

const players = ref([]);
const lastUpdated = ref("");
const tooltip = ref({
  show: false,
  x: 0,
  y: 0,
  width: 0,
  content: ''
});

function tierClass(tier) {
  switch (tier) {
    case "IRON":
      return "bg-gradient-to-r from-zinc-600/80 to-zinc-700/80 text-zinc-100";
    case "BRONZE":
      return "bg-gradient-to-r from-amber-700/80 to-orange-600/80 text-amber-100";
    case "SILVER":
      return "bg-gradient-to-r from-slate-300/90 to-slate-400/90 text-slate-800";
    case "GOLD":
      return "bg-gradient-to-r from-yellow-500/90 to-amber-400/90 text-yellow-950";
    case "PLATINUM":
      return "bg-gradient-to-r from-emerald-400/90 to-cyan-400/90 text-emerald-950";
    case "DIAMOND":
      return "bg-gradient-to-r from-blue-400/90 to-indigo-400/90 text-blue-950";
    case "MASTER":
      return "bg-gradient-to-r from-purple-600/90 to-fuchsia-500/90 text-purple-100";
    case "GRANDMASTER":
      return "bg-gradient-to-r from-red-600/90 to-rose-500/90 text-red-100";
    case "CHALLENGER":
      return "bg-gradient-to-r from-cyan-300/95 via-blue-300/95 to-indigo-300/95 text-blue-950";
    default:
      return "bg-gradient-to-r from-gray-600/80 to-gray-500/80 text-gray-100";
  }
}

function calculateWinrate(player) {
  const total = player.wins + player.losses;
  return total > 0 ? Math.round((player.wins / total) * 100) : 0;
}

function getTierPriority(tier) {
  const tierOrder = {
    'CHALLENGER': 8,
    'GRANDMASTER': 7,
    'MASTER': 6,
    'DIAMOND': 5,
    'PLATINUM': 4,
    'GOLD': 3,
    'SILVER': 2,
    'BRONZE': 1,
    'IRON': 0
  };
  return tierOrder[tier] ?? -1;
}

function getRankPriority(rank) {
  const rankOrder = { 'I': 4, 'II': 3, 'III': 2, 'IV': 1 };
  return rankOrder[rank] ?? 0;
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

function formatLastGameTime(lastMatchEndTime) {
  if (!lastMatchEndTime) return 'No recent matches';

  const date = new Date(Number(lastMatchEndTime));
  const now = new Date();
  const diffMs = now - date;
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  const localDateTime = date.toLocaleString('en-AU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    hour12: true
  });

  if (diffDays > 0) {
    return `${diffDays} day${diffDays > 1 ? 's' : ''} ago\n${localDateTime}`;
  } else if (diffHours > 0) {
    return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago\n${localDateTime}`;
  } else {
    const diffMins = Math.floor(diffMs / (1000 * 60));
    return `${diffMins} min${diffMins > 1 ? 's' : ''} ago\n${localDateTime}`;
  }
}

function showTooltip(event, player) {
  const container = event.currentTarget.closest('.w-\\[330px\\]');
  const containerRect = container.getBoundingClientRect();

  tooltip.value = {
    show: true,
    x: containerRect.left + 16,
    y: containerRect.top + 60, // Position near the top, below the header
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
        ? data.map(p => ({
          ...p,
          recently_played: isRecentlyPlayed(p.last_match_end_time)
        }))
        : [];

      const latest = players.value
        .map(p => new Date(p.updated_at.replace(" ", "T") + "Z"))
        .sort((a, b) => b - a)[0];

      lastUpdated.value = latest?.toISOString() ?? "";
    } else {
      players.value = [];
      lastUpdated.value = "";
    }
  } catch (err) {
    console.error("Failed to fetch ranks:", err);
    players.value = [];
    lastUpdated.value = "";
  }
}

function isRecentlyPlayed(lastMatchEndTime) {
  if (!lastMatchEndTime) return false;
  const now = Date.now();
  return now - Number(lastMatchEndTime) < 60 * 60 * 1000;
}

onMounted(async () => {
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
    opacity: 0.7;
  }
}

@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-shimmer {
  animation: shimmer 3s ease-in-out infinite;
}

.animate-pulse-slow {
  animation: pulse-slow 2s ease-in-out infinite;
}

.animate-fade-in {
  animation: fade-in 0.2s ease-out;
}

.drop-shadow-glow {
  filter: drop-shadow(0 0 8px rgba(251, 191, 36, 0.4));
}

/* Tooltip transitions */
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
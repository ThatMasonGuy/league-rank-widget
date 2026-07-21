<template>
  <div class="league-widget" data-tauri-drag-region>
    <div class="widget-ambient" aria-hidden="true"></div>

    <header class="widget-header" data-tauri-drag-region>
      <div class="brand" data-tauri-drag-region>
        <img
          class="brand-mark"
          :src="appIcon"
          alt=""
          aria-hidden="true"
          draggable="false"
        />
        <span>League Tracker</span>
      </div>

      <div class="header-meta">
        <button class="view-toggle" type="button" @click="toggleView">
          {{ viewMode === "current" ? "History" : "Ranks" }}
        </button>
        <span class="header-divider" aria-hidden="true"></span>
        <time class="clock">{{ formatClock(currentTime) }}</time>
      </div>
    </header>

    <main class="widget-content">
      <section
        v-if="viewMode === 'current' && playerCards.length"
        class="player-grid"
      >
        <article
          v-for="player in playerCards"
          :key="player.summoner_name"
          class="player-card"
          :class="[
            `status-${player.activity.kind}`,
            { 'is-unranked': player.isUnranked },
          ]"
          :style="getTierStyle(player.tier)"
          role="link"
          tabindex="0"
          :aria-label="`Open ${player.summoner_name} on OP.GG`"
          @click="openOpGG(player)"
          @keydown.enter="openOpGG(player)"
          @keydown.space.prevent="openOpGG(player)"
        >
          <div
            class="status-rail"
            :class="`rail-${player.activity.kind}`"
            aria-hidden="true"
          >
            <span class="status-dot"></span>
            <span class="status-line"></span>
            <span class="status-label">{{ player.activity.label }}</span>
          </div>

          <div class="card-body">
            <div class="card-top">
              <div class="identity-row">
                <div class="player-name" :title="player.summoner_name">
                  {{ player.summoner_name }}
                </div>
                <div class="last-active">{{ player.activity.lastActive }}</div>
              </div>

              <div class="rank-panel">
                <div class="rank-main">
                  <div class="rank-badge-slot" aria-hidden="true">
                    <svg
                      v-if="player.isUnranked"
                      class="unranked-icon"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    >
                      <circle cx="12" cy="12" r="10" />
                      <path d="m9 15 6-6" />
                    </svg>
                    <img
                      v-else-if="player.badgeUrl"
                      class="rank-badge-image"
                      :src="player.badgeUrl"
                      alt=""
                      draggable="false"
                      @error="hideBrokenBadge"
                    />
                  </div>

                  <div class="rank-lp">
                    <strong>{{ player.league_points ?? 0 }}</strong>
                    <span>LP</span>
                  </div>
                </div>

                <div class="rank-heading">{{ player.rankLabel }}</div>
              </div>
            </div>

            <div class="stats-row">
              <div class="stat">
                <span class="stat-label">Win rate</span>
                <strong :class="winRateTone(player.winRate)"
                  >{{ player.winRate }}%</strong
                >
              </div>
              <span class="stats-divider" aria-hidden="true"></span>
              <div class="stat stat-right">
                <span class="stat-label">Streak</span>
                <strong :class="streakTone(player)">{{
                  player.streakLabel
                }}</strong>
              </div>
            </div>

            <svg
              class="sparkline"
              viewBox="0 0 170 25"
              preserveAspectRatio="none"
              aria-hidden="true"
            >
              <defs>
                <linearGradient
                  :id="`spark-${player.safeId}`"
                  x1="0"
                  y1="0"
                  x2="0"
                  y2="1"
                >
                  <stop
                    offset="0%"
                    stop-color="var(--tier-accent)"
                    stop-opacity="0.24"
                  />
                  <stop
                    offset="100%"
                    stop-color="var(--tier-accent)"
                    stop-opacity="0"
                  />
                </linearGradient>
              </defs>
              <polygon
                v-if="player.sparkline.length > 1"
                :points="`${player.sparkline.map((point) => `${point.x},${point.y}`).join(' ')} 170,25 0,25`"
                :fill="`url(#spark-${player.safeId})`"
              />
              <polyline
                :points="
                  player.sparkline
                    .map((point) => `${point.x},${point.y}`)
                    .join(' ')
                "
                fill="none"
                stroke="var(--tier-accent)"
                stroke-width="1.8"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
              <circle
                v-for="(point, index) in player.sparkline"
                :key="`${player.safeId}-${index}`"
                :cx="point.x"
                :cy="point.y"
                :r="index === player.sparkline.length - 1 ? 1.9 : 1.2"
                fill="var(--tier-accent)"
              />
            </svg>

            <div class="movement-header">
              <span>Recent movement</span>
              <button
                class="opgg-link"
                type="button"
                @click.stop="openOpGG(player)"
              >
                OP.GG
                <span class="external-link-mark" aria-hidden="true"></span>
              </button>
            </div>

            <div class="movement-grid">
              <div
                v-for="movement in player.movements"
                :key="movement.key"
                class="movement-item"
                :class="{ blank: movement.delta === null }"
              >
                <span class="movement-age">{{ movement.label }}</span>
                <strong
                  v-if="movement.delta !== null"
                  :class="
                    movement.delta > 0
                      ? 'positive'
                      : movement.delta < 0
                        ? 'negative'
                        : 'neutral'
                  "
                >
                  {{ movement.delta > 0 ? "+" : "" }}{{ movement.delta }} LP
                </strong>
                <span v-else class="movement-placeholder">—</span>
              </div>
            </div>
          </div>
        </article>
      </section>

      <section v-else-if="viewMode === 'history'" class="history-view">
        <div class="history-heading">
          <div>
            <span class="history-kicker">Rank movement</span>
            <strong>Recent changes</strong>
          </div>
          <span class="history-updated"
            >Updated {{ formatUpdated(lastUpdated) }}</span
          >
        </div>

        <div v-if="historyRows.length" class="history-list">
          <button
            v-for="row in historyRows"
            :key="row.key"
            class="history-row"
            type="button"
            :style="getTierStyle(row.tier)"
            @click="openOpGG(row.player)"
          >
            <span class="history-accent"></span>
            <span class="history-player">
              <strong>{{ row.name }}</strong>
              <small>{{ row.age }}</small>
            </span>
            <span class="history-rank">{{ row.rankLabel }}</span>
            <span
              class="history-delta"
              :class="row.delta > 0 ? 'positive' : 'negative'"
            >
              {{ row.delta > 0 ? "+" : "" }}{{ row.delta }} LP
            </span>
          </button>
        </div>

        <div v-else class="history-empty-state">
          <strong>No meaningful changes yet</strong>
          <span
            >The tracker has snapshots, but nothing moved. Riveting stuff.</span
          >
        </div>
      </section>

      <section v-else class="loading-state">
        <span class="loading-mark" aria-hidden="true"></span>
        <strong>Loading ranks</strong>
        <span>Waiting for the API to remember it has one job.</span>
      </section>
    </main>
  </div>
</template>

<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import ironBadge from "./assets/ranks/iron.png";
import bronzeBadge from "./assets/ranks/bronze.png";
import silverBadge from "./assets/ranks/silver.png";
import goldBadge from "./assets/ranks/gold.png";
import platinumBadge from "./assets/ranks/platinum.png";
import emeraldBadge from "./assets/ranks/emerald.png";
import diamondBadge from "./assets/ranks/diamond.png";
import masterBadge from "./assets/ranks/master.png";
import grandmasterBadge from "./assets/ranks/grandmaster.png";
import challengerBadge from "./assets/ranks/challenger.png";
import appIcon from "./assets/icon.png";

const badgeUrls = {
  IRON: ironBadge,
  BRONZE: bronzeBadge,
  SILVER: silverBadge,
  GOLD: goldBadge,
  PLATINUM: platinumBadge,
  EMERALD: emeraldBadge,
  DIAMOND: diamondBadge,
  MASTER: masterBadge,
  GRANDMASTER: grandmasterBadge,
  CHALLENGER: challengerBadge,
};

const tierOrder = [
  "IRON",
  "BRONZE",
  "SILVER",
  "GOLD",
  "PLATINUM",
  "EMERALD",
  "DIAMOND",
  "MASTER",
  "GRANDMASTER",
  "CHALLENGER",
];

const rankOrder = { IV: 0, III: 1, II: 2, I: 3 };
const movementPeriods = [
  { label: "2D", days: 2 },
  { label: "4D", days: 4 },
  { label: "18D", days: 18 },
];

const tierThemes = {
  IRON: ["#a7a7a7", "rgba(167, 167, 167, 0.14)", "rgba(167, 167, 167, 0.32)"],
  BRONZE: ["#d58a5d", "rgba(213, 138, 93, 0.14)", "rgba(213, 138, 93, 0.34)"],
  SILVER: ["#8eb8e8", "rgba(142, 184, 232, 0.13)", "rgba(142, 184, 232, 0.34)"],
  GOLD: ["#e8bd5c", "rgba(232, 189, 92, 0.14)", "rgba(232, 189, 92, 0.34)"],
  PLATINUM: ["#5fd2cf", "rgba(95, 210, 207, 0.13)", "rgba(95, 210, 207, 0.34)"],
  EMERALD: ["#4bdc92", "rgba(75, 220, 146, 0.15)", "rgba(75, 220, 146, 0.38)"],
  DIAMOND: [
    "#7fa9ff",
    "rgba(127, 169, 255, 0.14)",
    "rgba(127, 169, 255, 0.36)",
  ],
  MASTER: ["#c782ff", "rgba(199, 130, 255, 0.14)", "rgba(199, 130, 255, 0.36)"],
  GRANDMASTER: [
    "#ff736c",
    "rgba(255, 115, 108, 0.14)",
    "rgba(255, 115, 108, 0.36)",
  ],
  CHALLENGER: [
    "#65d9ff",
    "rgba(101, 217, 255, 0.14)",
    "rgba(101, 217, 255, 0.38)",
  ],
  UNRANKED: [
    "#76808e",
    "rgba(118, 128, 142, 0.08)",
    "rgba(118, 128, 142, 0.22)",
  ],
};

const players = ref([]);
const history = ref([]);
const lastUpdated = ref("");
const viewMode = ref("current");
const currentTime = ref(new Date());

let rankRefreshTimer;
let historyRefreshTimer;
let clockTimer;

const historyByPlayer = computed(() => {
  const grouped = new Map();

  for (const entry of history.value) {
    const name = entry?.summoner_name;
    if (!name) continue;
    if (!grouped.has(name)) grouped.set(name, []);
    grouped.get(name).push(entry);
  }

  for (const entries of grouped.values()) {
    entries.sort(
      (a, b) => parseTimestamp(b.updated_at) - parseTimestamp(a.updated_at),
    );
  }

  return grouped;
});

const playerCards = computed(() =>
  sortPlayers(players.value).map((player) => {
    const tier = normalizeTier(player.tier);
    const movements = getRecentMovements(player);

    return {
      ...player,
      tier,
      isUnranked: tier === "UNRANKED",
      rankLabel: formatRank(player),
      badgeUrl: badgeUrls[tier] ?? null,
      winRate: calculateWinRate(player),
      streakLabel: formatStreak(player),
      activity: getActivity(player),
      movements,
      sparkline: getSparkline(player),
      safeId: String(player.summoner_name || "player").replace(
        /[^a-zA-Z0-9_-]/g,
        "-",
      ),
    };
  }),
);

const historyRows = computed(() => {
  const rows = [];
  const playerLookup = new Map(
    players.value.map((player) => [player.summoner_name, player]),
  );

  for (const [name, entries] of historyByPlayer.value.entries()) {
    const snapshots = collapseUnchanged(entries);

    for (let index = 0; index < snapshots.length - 1; index += 1) {
      const current = snapshots[index];
      const previous = snapshots[index + 1];
      const delta = rankScore(current) - rankScore(previous);
      if (!delta) continue;

      rows.push({
        key: `${name}-${current.updated_at}-${index}`,
        name,
        player: playerLookup.get(name) ?? current,
        tier: normalizeTier(current.tier),
        rankLabel: formatRank(current),
        delta,
        timestamp: parseTimestamp(current.updated_at),
        age: formatCompactAge(current.updated_at),
      });
    }
  }

  return rows.sort((a, b) => b.timestamp - a.timestamp).slice(0, 18);
});

function toggleView() {
  viewMode.value = viewMode.value === "current" ? "history" : "current";
}

async function openOpGG(player) {
  if (!player?.summoner_name) return;
  const name = encodeURIComponent(`${player.summoner_name}-OC`);
  const url = `https://www.op.gg/summoners/oce/${name}`;

  try {
    await openUrl(url);
  } catch (error) {
    console.error("Failed to open OP.GG:", error);
  }
}

function hideBrokenBadge(event) {
  event.currentTarget.hidden = true;
}

function normalizeTier(tier) {
  const normalized = String(tier || "UNRANKED").toUpperCase();
  return tierOrder.includes(normalized) ? normalized : "UNRANKED";
}

function formatRank(player) {
  const tier = normalizeTier(player?.tier);
  if (tier === "UNRANKED") return "UNRANKED";
  if (["MASTER", "GRANDMASTER", "CHALLENGER"].includes(tier)) return tier;
  return `${tier} ${player?.rank || "IV"}`;
}

function calculateWinRate(player) {
  const wins = Number(player?.wins) || 0;
  const losses = Number(player?.losses) || 0;
  const total = wins + losses;
  return total ? Math.round((wins / total) * 100) : 0;
}

function formatStreak(player) {
  const winStreak = Number(player?.win_streak) || 0;
  const lossStreak = Number(player?.loss_streak) || 0;
  if (winStreak > 0) return `${winStreak}W`;
  if (lossStreak > 0) return `${lossStreak}L`;
  return "—";
}

function streakTone(player) {
  if ((Number(player?.win_streak) || 0) > 0) return "positive";
  if ((Number(player?.loss_streak) || 0) > 0) return "negative";
  return "neutral";
}

function winRateTone(winRate) {
  if (winRate >= 52) return "positive";
  if (winRate <= 47) return "negative";
  return "neutral";
}

function getActivity(player) {
  const explicitlyInGame = Boolean(
    player?.in_game ||
    player?.is_in_game ||
    player?.active_now ||
    player?.status === "in_game" ||
    player?.status === "active",
  );

  const lastMatch = Number(player?.last_match_end_time) || 0;
  const elapsed = lastMatch ? Date.now() - lastMatch : Number.POSITIVE_INFINITY;
  const lastActive = formatLastPlayed(lastMatch);

  if (explicitlyInGame)
    return { kind: "live", label: "IN GAME", lastActive: "Active now" };
  if (elapsed <= 60 * 60 * 1000)
    return { kind: "recent", label: "RECENT", lastActive };
  return { kind: "offline", label: "OFFLINE", lastActive };
}

function getTierStyle(tier) {
  const [accent, soft, border] =
    tierThemes[normalizeTier(tier)] ?? tierThemes.UNRANKED;
  return {
    "--tier-accent": accent,
    "--tier-soft": soft,
    "--tier-border": border,
  };
}

function rankScore(entry) {
  const tier = normalizeTier(entry?.tier);
  if (tier === "UNRANKED") return -100;

  const tierIndex = tierOrder.indexOf(tier);
  const division = rankOrder[String(entry?.rank || "").toUpperCase()] ?? 0;
  const lp = Number(entry?.league_points) || 0;

  return tierIndex * 400 + division * 100 + lp;
}

function collapseUnchanged(entries) {
  const collapsed = [];

  for (const entry of entries) {
    if (
      !collapsed.length ||
      rankScore(entry) !== rankScore(collapsed[collapsed.length - 1])
    ) {
      collapsed.push(entry);
    }
  }

  return collapsed;
}

function getSnapshots(player) {
  const entries = historyByPlayer.value.get(player.summoner_name) ?? [];
  const snapshots = [player, ...entries]
    .filter(Boolean)
    .sort(
      (a, b) => parseTimestamp(b.updated_at) - parseTimestamp(a.updated_at),
    );

  const deduped = [];
  const seen = new Set();

  for (const snapshot of snapshots) {
    const key = `${snapshot.updated_at || ""}-${rankScore(snapshot)}`;
    if (seen.has(key)) continue;
    seen.add(key);
    deduped.push(snapshot);
  }

  return deduped;
}

function getRecentMovements(player) {
  const snapshots = getSnapshots(player)
    .map((snapshot) => ({
      snapshot,
      timestamp: parseTimestamp(snapshot.updated_at),
    }))
    .filter((entry) => entry.timestamp)
    .sort((a, b) => b.timestamp - a.timestamp);

  const currentTier = normalizeTier(player?.tier);
  const currentScore = rankScore(player);
  const now = Date.now();

  return movementPeriods.map(({ label, days }) => {
    if (currentTier === "UNRANKED") {
      return { key: label, label, delta: null };
    }

    const cutoff = now - days * 24 * 60 * 60 * 1000;
    const baseline = snapshots.find((entry) => entry.timestamp <= cutoff);

    if (!baseline || normalizeTier(baseline.snapshot?.tier) === "UNRANKED") {
      return { key: label, label, delta: null };
    }

    return {
      key: label,
      label,
      delta: currentScore - rankScore(baseline.snapshot),
    };
  });
}

function getSparkline(player) {
  const snapshots = collapseUnchanged(getSnapshots(player))
    .slice(0, 7)
    .reverse();
  const values = snapshots.map(rankScore);

  if (!values.length)
    return [
      { x: 0, y: 12.5 },
      { x: 170, y: 12.5 },
    ];
  if (values.length === 1)
    return [
      { x: 0, y: 12.5 },
      { x: 170, y: 12.5 },
    ];

  const min = Math.min(...values);
  const max = Math.max(...values);
  const range = Math.max(max - min, 1);

  return values.map((value, index) => ({
    x: Number(((index / (values.length - 1)) * 170).toFixed(2)),
    y: Number((22 - ((value - min) / range) * 18).toFixed(2)),
  }));
}

function sortPlayers(list) {
  return [...list].sort((a, b) => rankScore(b) - rankScore(a));
}

function parseTimestamp(value) {
  if (!value) return 0;
  if (typeof value === "number" || /^\d+$/.test(String(value))) {
    const numeric = Number(value);
    return numeric < 10_000_000_000 ? numeric * 1000 : numeric;
  }

  const raw = String(value);
  const normalized = raw.includes("T") ? raw : `${raw.replace(" ", "T")}Z`;
  const parsed = Date.parse(normalized);
  return Number.isNaN(parsed) ? 0 : parsed;
}

function formatLastPlayed(timestamp) {
  if (!timestamp) return "No games yet";
  return formatAgeFromMs(Date.now() - timestamp, false);
}

function formatCompactAge(value) {
  const timestamp = parseTimestamp(value);
  if (!timestamp) return "—";
  return formatAgeFromMs(Date.now() - timestamp, true);
}

function formatAgeFromMs(diffMs, compact) {
  const safeDiff = Math.max(0, diffMs);
  const minutes = Math.floor(safeDiff / 60_000);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (compact) {
    if (days > 0) return `${days}D`;
    if (hours > 0) return `${hours}H`;
    if (minutes > 0) return `${minutes}M`;
    return "NOW";
  }

  if (days > 0) return `${days}d ago`;
  if (hours > 0) return `${hours}h ago`;
  if (minutes > 0) return `${minutes}m ago`;
  return "Just now";
}

function formatClock(date) {
  return date.toLocaleTimeString([], { hour: "numeric", minute: "2-digit" });
}

function formatUpdated(value) {
  const timestamp = parseTimestamp(value);
  if (!timestamp) return "—";
  return formatAgeFromMs(Date.now() - timestamp, false);
}

async function fetchRanks() {
  try {
    const response = await fetch("https://lol.mxn.au/ranks");
    if (!response.ok) throw new Error(`HTTP ${response.status}`);

    const data = await response.json();
    players.value = Array.isArray(data) ? data : [];

    const latest = players.value
      .map((player) => parseTimestamp(player.updated_at))
      .filter(Boolean)
      .sort((a, b) => b - a)[0];

    lastUpdated.value = latest ? new Date(latest).toISOString() : "";
  } catch (error) {
    console.error("Failed to fetch ranks:", error);
  }
}

async function fetchHistory() {
  try {
    const response = await fetch("https://lol.mxn.au/ranks/all");
    if (!response.ok) throw new Error(`HTTP ${response.status}`);

    const data = await response.json();
    history.value = Array.isArray(data) ? data : [];
  } catch (error) {
    console.error("Failed to fetch rank history:", error);
  }
}

onMounted(() => {
  fetchRanks();
  fetchHistory();

  rankRefreshTimer = window.setInterval(fetchRanks, 60_000);
  historyRefreshTimer = window.setInterval(fetchHistory, 5 * 60_000);
  clockTimer = window.setInterval(() => {
    currentTime.value = new Date();
  }, 30_000);
});

onBeforeUnmount(() => {
  window.clearInterval(rankRefreshTimer);
  window.clearInterval(historyRefreshTimer);
  window.clearInterval(clockTimer);
});
</script>

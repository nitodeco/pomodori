import type { SessionStats } from "../database";
import { getStats } from "../database";

type StatsSubscriber = (stats: SessionStats) => void;

const DEFAULT_STATS: SessionStats = {
  totalSessions: 0,
  completedSessions: 0,
  totalWorkTimeInSecs: 0,
  totalBreakTimeInSecs: 0,
};

const getStartOfDay = (date: Date): string => {
  const startOfDay = new Date(date);
  startOfDay.setHours(0, 0, 0, 0);

  return startOfDay.toISOString();
};

const getEndOfDay = (date: Date): string => {
  const endOfDay = new Date(date);
  endOfDay.setHours(23, 59, 59, 999);

  return endOfDay.toISOString();
};

const createStatsStore = () => {
  let todayStats: SessionStats = DEFAULT_STATS;
  let allTimeStats: SessionStats = DEFAULT_STATS;
  const todaySubscribers = new Set<StatsSubscriber>();
  const allTimeSubscribers = new Set<StatsSubscriber>();

  const notifyTodaySubscribers = (): void => {
    for (const subscriber of todaySubscribers) {
      subscriber(todayStats);
    }
  };

  const notifyAllTimeSubscribers = (): void => {
    for (const subscriber of allTimeSubscribers) {
      subscriber(allTimeStats);
    }
  };

  const refreshTodayStats = async (): Promise<SessionStats> => {
    const today = new Date();
    const fromDate = getStartOfDay(today);
    const toDate = getEndOfDay(today);

    todayStats = await getStats(fromDate, toDate);
    notifyTodaySubscribers();

    return todayStats;
  };

  const refreshAllTimeStats = async (): Promise<SessionStats> => {
    allTimeStats = await getStats();
    notifyAllTimeSubscribers();

    return allTimeStats;
  };

  const refresh = async (): Promise<void> => {
    await Promise.all([refreshTodayStats(), refreshAllTimeStats()]);
  };

  const getTodayStats = (): SessionStats => todayStats;

  const getAllTimeStats = (): SessionStats => allTimeStats;

  const subscribeToday = (subscriber: StatsSubscriber): (() => void) => {
    todaySubscribers.add(subscriber);
    subscriber(todayStats);

    return () => {
      todaySubscribers.delete(subscriber);
    };
  };

  const subscribeAllTime = (subscriber: StatsSubscriber): (() => void) => {
    allTimeSubscribers.add(subscriber);
    subscriber(allTimeStats);

    return () => {
      allTimeSubscribers.delete(subscriber);
    };
  };

  const cleanup = (): void => {
    todaySubscribers.clear();
    allTimeSubscribers.clear();
    todayStats = DEFAULT_STATS;
    allTimeStats = DEFAULT_STATS;
  };

  return {
    refresh,
    refreshTodayStats,
    refreshAllTimeStats,
    getTodayStats,
    getAllTimeStats,
    subscribeToday,
    subscribeAllTime,
    cleanup,
  };
};

export const statsStore = createStatsStore();

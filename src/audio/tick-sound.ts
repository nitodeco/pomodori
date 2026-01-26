type TickSoundConfig = {
  frequencyInHz?: number;
  durationInMs?: number;
  volumePercent?: number;
};

const DEFAULT_FREQUENCY_IN_HZ = 800;
const DEFAULT_DURATION_IN_MS = 50;
const DEFAULT_VOLUME_PERCENT = 15;

export const createTickSound = (config: TickSoundConfig = {}) => {
  const {
    frequencyInHz = DEFAULT_FREQUENCY_IN_HZ,
    durationInMs = DEFAULT_DURATION_IN_MS,
    volumePercent = DEFAULT_VOLUME_PERCENT,
  } = config;

  let maybeAudioContext: AudioContext | null = null;

  const getAudioContext = (): AudioContext => {
    if (!maybeAudioContext) {
      maybeAudioContext = new AudioContext();
    }

    return maybeAudioContext;
  };

  const play = () => {
    const audioContext = getAudioContext();

    if (audioContext.state === "suspended") {
      audioContext.resume();
    }

    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();

    oscillator.type = "sine";
    oscillator.frequency.value = frequencyInHz;

    const volume = volumePercent / 100;
    const durationInSecs = durationInMs / 1_000;

    gainNode.gain.setValueAtTime(volume, audioContext.currentTime);
    gainNode.gain.exponentialRampToValueAtTime(
      0.001,
      audioContext.currentTime + durationInSecs
    );

    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);

    oscillator.start(audioContext.currentTime);
    oscillator.stop(audioContext.currentTime + durationInSecs);
  };

  const destroy = () => {
    if (maybeAudioContext) {
      maybeAudioContext.close();
      maybeAudioContext = null;
    }
  };

  return {
    play,
    destroy,
  };
};

export type TickSound = ReturnType<typeof createTickSound>;

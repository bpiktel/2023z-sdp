export const getAudioPath = (audioId: string): string =>
  `${import.meta.env.VITE_BASE_API_URL}/audio/${audioId}`;

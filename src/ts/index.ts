import * as emotes from './emotes';
import type { EmoteData } from './emotes';

function parseJson(callback: () => string) {
  return () => JSON.parse(callback());
}

export const parseString = emotes.parseString;

export const getBTTV: () => EmoteData[] = parseJson(emotes.getBTTV);

import * as emotes from './emotes';
import type { EmoteData } from './emotes';

function parseJson(callback: (...args: any[]) => string) {
  return (...args: any[]) => JSON.parse(callback(args));
}

export const parseString = emotes.parseString;

export const getBTTV: () => EmoteData[] = parseJson(emotes.getBTTV);

import * as emotes from './emotes';

function parseJson(callback: () => string) {
  return () => JSON.parse(callback());
}

export const parseString = emotes.parseString;

export const getBTTV = parseJson(emotes.getBTTV);

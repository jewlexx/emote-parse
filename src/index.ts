import * as emotes from './emotes';

function parseJson(callback: () => string) {
  return () => JSON.parse(callback());
}

export type ParseReturn = {
  id: string;
  code: string;
  index: number;
  userId: string;
  imageType: 'png' | 'jpg';
}[];

export function parseString(str: string, emotes: any[]): ParseReturn {
  const numbers: ParseReturn = [];

  emotes.forEach((emote) => {
    function getIndex(startIndex?: number) {
      const index = str.indexOf(emote.code, startIndex);

      if (index != -1) {
        numbers.push({
          ...emote,
          index,
        });
        getIndex(index + 1);
      }
    }

    getIndex();
  });

  return numbers;
}

export const getBTTV = parseJson(emotes.getBTTV);

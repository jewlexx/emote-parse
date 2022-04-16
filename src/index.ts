import * as emotes from './emotes';

function parseJson(callback: () => string) {
  return () => JSON.parse(callback());
}

type EmoteUrl = `https://cdn.betterttv.net/emote/${string}/${1 | 2 | 3}x`;

export type ParseReturn = {
  id: string;
  code: string;
  index: number;
  userId: string;
  urls: EmoteUrl[];
  imageType: 'png' | 'jpg';
}[];

export function parseString(str: string, emotes: any[]): ParseReturn {
  const numbers: ParseReturn = [];

  emotes.forEach((emote) => {
    const urls = [1, 2, 3].map((v) => {
      return `https://cdn.betterttv.net/emote/${emote.id}/${v}x`;
    });

    function getIndex(startIndex?: number) {
      const index = str.indexOf(emote.code, startIndex);

      if (index != -1) {
        numbers.push({
          ...emote,
          urls,
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

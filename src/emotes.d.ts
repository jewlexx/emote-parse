export interface EmoteData {
  id: number;
  code: string;
  imageType: 'png' | 'gif';
  userId: string;
}

export type ParseReturn = {
  emote: EmoteData;
  index: number;
  urls: EmoteUrl[];
};

export type EmoteUrl = `https://cdn.betterttv.net/emote/${string}/${
  | 1
  | 2
  | 3}x`;

export declare function getBTTV(): string;
export declare function parseString(str: string, emotes: any[]): ParseReturn[];

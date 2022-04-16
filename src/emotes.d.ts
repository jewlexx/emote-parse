export type ParseReturn = {
  id: string;
  code: string;
  index: number;
  userId: string;
  urls: EmoteUrl[];
  imageType: 'png' | 'gif';
}[];

export type EmoteUrl = `https://cdn.betterttv.net/emote/${string}/${
  | 1
  | 2
  | 3}x`;

export declare function getBTTV(): string;
export declare function parseString(str: string, emotes: any[]): ParseReturn;

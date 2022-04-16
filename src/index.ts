import * as emotes from './emotes';

function parseJson(callback: (...args: any) => string) {
  return () => JSON.parse(callback());
}

function parseString(str: string, emotes: any[]): number[] {
  const numbers: number[] = [];

  emotes.forEach((emote) => {
    function getIndex(startIndex?: number) {
      const index = str.indexOf(emote.code, startIndex);

      if (index != -1) {
        numbers.push(index);
        getIndex(index + 1);
      }
    }

    getIndex();
  });

  return numbers;
}

type EmotesModule = typeof emotes;
interface IModule extends EmotesModule {
  /** Returns an array of all the indexes of the emotes in the string */
  parseString: (str: string, emotes: any) => number[];
  [key: string]: any;
}

const parser: IModule = {
  getBTTV: parseJson(emotes.getBTTV),
  parseString,
};

export = parser;

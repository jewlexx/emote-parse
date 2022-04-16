import * as emotes from './emotes';

function parseJson(callback: () => string) {
  return () => JSON.parse(callback());
}

// export function parseString(str: string, emotes: any[]): ParseReturn {
//   const numbers: ParseReturn = [];

//   emotes.forEach((emote) => {
//     const urls = [1, 2, 3].map((v) => {
//       return `https://cdn.betterttv.net/emote/${emote.id}/${v}x`;
//     });

//     function getIndex(startIndex?: number) {
//       const index = str.indexOf(emote.code, startIndex);

//       if (index != -1) {
//         numbers.push({
//           ...emote,
//           urls,
//           index,
//         });
//         getIndex(index + 1);
//       }
//     }

//     getIndex();
//   });

//   return numbers;
// }

export const parseString = emotes.parseString;

export const getBTTV = parseJson(emotes.getBTTV);

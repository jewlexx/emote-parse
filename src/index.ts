import * as emotes from './emotes';

function parseJson(callback: (...args: any) => string) {
  return () => JSON.parse(callback());
}

type Module = typeof emotes;

const module: Module = {
  getBTTV: parseJson(emotes.getBTTV),
};

export = module;

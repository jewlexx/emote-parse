const neon = require('../');

const bttv = neon.getBTTV();
const ex = neon.parseString(':tf: :tf: CiGrip', bttv);
console.log(ex);

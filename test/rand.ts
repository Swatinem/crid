const MAX = 0x7fffffff;
export default () => Math.floor(Math.random() * MAX) & MAX;

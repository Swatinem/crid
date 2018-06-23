declare module "jsverify" {
  interface AssertForall {
    (arg: string, fn: (arg: any) => void): void;
    (arg1: string, arg2: string, fn: (arg1: any, arg2: any) => void): void;
  }
  var assertForall: AssertForall;
  var utils: {
    isEqual<T>(a: T, b: T): boolean;
  };
  export { assertForall, utils };
}

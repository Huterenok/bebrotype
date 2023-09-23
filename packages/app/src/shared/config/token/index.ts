import { createEvent, createStore, sample } from "effector";
import persist from "effector-localstorage";
import { createFactory } from "@withease/factories";

interface createTokenParams<T> {
  tokenIdent: string;
  defaultValue: any;
}

export const createToken = <T>(params: createTokenParams<T>) => {
  return createFactory(() => {
    const setToken = createEvent<T>();
    const $token = createStore<T>(params.defaultValue);

    sample({
      clock: setToken,
      target: $token,
    });

    persist({
      store: $token,
      key: params.tokenIdent,
    });

    return {
      $token,
      setToken,
    };
  })();
};
